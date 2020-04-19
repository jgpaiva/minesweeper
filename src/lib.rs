#![recursion_limit = "512"]

use std::collections::HashMap;

use lib_minesweeper::create_board;
use lib_minesweeper::numbers_on_board;
use lib_minesweeper::Board;
use lib_minesweeper::BoardState::Failed;
use lib_minesweeper::BoardState::NotReady;
use lib_minesweeper::BoardState::Playing;
use lib_minesweeper::BoardState::Ready;
use lib_minesweeper::BoardState::Won;
use lib_minesweeper::MapElement::Mine;
use lib_minesweeper::MapElement::Number;
use lib_minesweeper::MapElementCellState::Closed;
use lib_minesweeper::MapElementCellState::Flagged;
use lib_minesweeper::Point;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use serde_derive::{Deserialize, Serialize};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

#[derive(Debug, PartialEq)]
struct SvgSquare {
    props: HashMap<String, String>,
}

fn small_board() -> Board {
    use rand::Rng;
    let width = 10;
    let height = 10;
    let mines = 10;

    let board = create_board(width, height, mines, |x, y| {
        rand::thread_rng().gen_range(x, y)
    });

    numbers_on_board(board)
}

fn medium_board() -> Board {
    use rand::Rng;
    let width = 16;
    let height = 16;
    let mines = 40;

    let board = create_board(width, height, mines, |x, y| {
        rand::thread_rng().gen_range(x, y)
    });

    numbers_on_board(board)
}

fn large_board() -> Board {
    use rand::Rng;
    let width = 16;
    let height = 30;
    let mines = 99;

    let board = create_board(width, height, mines, |x, y| {
        rand::thread_rng().gen_range(x, y)
    });

    numbers_on_board(board)
}

#[derive(Debug, PartialEq)]
struct CellItem {
    props: HashMap<String, String>,
}

fn create_item(width: usize, _height: usize) -> CellItem {
    let mut props = HashMap::new();
    let square_size: f64 = 100.0 / (width as f64);
    let margin: f64 = 0.05 * square_size;
    let width = format!("{:.2}", square_size - 2.0 * margin);

    let style = format!("width: {}%; margin: {}%", width, margin);
    props.insert("style".to_string(), style);
    props.insert("class".to_string(), "item".to_string());
    CellItem { props }
}

#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum Mode {
    Flagging,
    Digging,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

lazy_static! {
    static ref BOARD: Mutex<Board> = Mutex::new(small_board());
    static ref MODE: Mutex<Mode> = Mutex::new(Mode::Digging);
    static ref DIFFICULTY: Mutex<Difficulty> = Mutex::new(Difficulty::Easy);
}

fn update_board(p: Point) {
    {
        let mut board = BOARD.lock().unwrap();
        let mode = MODE.lock().unwrap();
        match *mode {
            Mode::Digging => {
                let val = board.cascade_open_item(&p);
                if let Some(new_board) = val {
                    *board = new_board;
                }
            }
            Mode::Flagging => {
                *board = board.flag_item(&p);
            }
        }
    }
    render_page().expect("should be able to create a new board");
}

fn toggle_mode() {
    {
        let mut mode = MODE.lock().unwrap();
        *mode = match *mode {
            Mode::Flagging => Mode::Digging,
            Mode::Digging => Mode::Flagging,
        }
    }
    render_page().expect("should be able to create a new board");
}

fn toggle_difficulty() {
    {
        let mut board = BOARD.lock().unwrap();
        let mut diff = DIFFICULTY.lock().unwrap();
        let (new_board, new_diff) = match (&board.state, diff.clone()) {
            (Ready, Difficulty::Easy) => (medium_board(), Difficulty::Medium),
            (Ready, Difficulty::Medium) => (large_board(), Difficulty::Hard),
            (Ready, Difficulty::Hard) => (small_board(), Difficulty::Easy),
            (_, Difficulty::Easy) => (small_board(), Difficulty::Easy),
            (_, Difficulty::Medium) => (medium_board(), Difficulty::Medium),
            (_, Difficulty::Hard) => (large_board(), Difficulty::Hard),
        };
        *board = new_board;
        *diff = new_diff;
    }
    render_page().expect("should be able to create a new board");
}

fn create_structure() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let div = document.create_element("div")?;
    div.set_attribute("class", "flex-container")?;
    div.set_attribute("id", "difficulty_button_placeholder")?;
    let button = document.create_element("div")?;
    button.set_attribute("id", "difficulty-button")?;
    button.set_attribute("class", "clickable item")?;
    div.append_child(&button).unwrap();
    body.append_child(&div).unwrap();

    let div = document.create_element("div")?;
    div.set_attribute("class", "flex-container")?;
    div.set_attribute("id", "mode_button_placeholder")?;
    let button = document.create_element("div")?;
    button.set_attribute("id", "mode-button")?;
    button.set_attribute("class", "item")?;
    div.append_child(&button).unwrap();
    body.append_child(&div).unwrap();

    let div = document.create_element("div")?;
    div.set_attribute("id", "board_game_placeholder")?;
    let board_game = document.create_element("div")?;
    board_game.set_attribute("id", "board_game")?;
    board_game.set_attribute("class", "flex-container")?;
    div.append_child(&board_game).unwrap();
    body.append_child(&div).unwrap();
    Ok(())
}

pub fn render_page() -> Result<(), JsValue> {
    let board = BOARD.lock().unwrap();
    let mode = MODE.lock().unwrap();
    let difficulty = DIFFICULTY.lock().unwrap();
    let is_done = matches!(board.state, Failed | Won);
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    match board.state {
        Ready | Playing => body.set_attribute("class", "ongoing"),
        Won => body.set_attribute("class", "won"),
        Failed => body.set_attribute("class", "failed"),
        NotReady => unreachable!(),
    }?;

    let mode_button = document.get_element_by_id("mode-button").unwrap();
    let div = mode_button.parent_node().unwrap();
    div.remove_child(&mode_button)
        .expect("should be able to remove this item");
    let mode_button = document.create_element("div")?;
    mode_button.set_attribute("id", "mode-button")?;
    mode_button.set_attribute("class", if is_done { "item" } else { "clickable item" })?;
    let img = document.create_element("img")?;
    img.set_attribute("style", "width: 2em; height:2em")?;
    let button_image = match (&board.state, mode.clone()) {
        (Ready, Mode::Flagging) | (Playing, Mode::Flagging) => "svg/flag.svg",
        (Ready, Mode::Digging) | (Playing, Mode::Digging) => "svg/dig.svg",
        (Won, _) => "svg/trophy.svg",
        (Failed, _) => "svg/skull.svg",
        _ => unreachable!(),
    };
    let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        toggle_mode();
    }) as Box<dyn FnMut(_)>);
    mode_button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();
    img.set_attribute("src", button_image)?;
    mode_button.append_child(&img).unwrap();
    div.append_child(&mode_button).unwrap();

    let difficulty_button = document.get_element_by_id("difficulty-button").unwrap();
    let div = difficulty_button.parent_node().unwrap();
    div.remove_child(&difficulty_button)
        .expect("should be able to remove this item");
    let difficulty_button = document.create_element("div")?;
    difficulty_button.set_attribute("id", "difficulty-button")?;
    difficulty_button.set_attribute("class", "clickable item")?;
    let button_contents = match difficulty.clone() {
        Difficulty::Easy => "😀",
        Difficulty::Medium => "🤨",
        Difficulty::Hard => "🧐",
    };
    difficulty_button.set_inner_html(button_contents);
    let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        toggle_difficulty();
    }) as Box<dyn FnMut(_)>);
    difficulty_button
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();
    div.append_child(&difficulty_button).unwrap();

    let div = document.get_element_by_id("board_game").unwrap();
    let board_game_placeholder = div.parent_node().unwrap();
    board_game_placeholder
        .remove_child(&div)
        .expect("should be able to remove this item");

    let div = document.create_element("div")?;
    div.set_attribute("id", "board_game")?;
    div.set_attribute("class", "flex-container")?;

    for y in 0..board.height {
        for x in 0..board.width {
            let x = x as i32;
            let y = y as i32;
            let inner_div = document.create_element("div")?;
            if !is_done {
                let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                    update_board(Point { x, y });
                }) as Box<dyn FnMut(_)>);
                inner_div
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
                closure.forget();
            }
            let item = create_item(board.width, board.height);
            for (k, v) in item.props {
                inner_div.set_attribute(&k, &v)?;
            }
            inner_div.set_attribute(
                "class",
                if is_done {
                    "item frozen"
                } else {
                    "item active"
                },
            )?;
            div.append_child(&inner_div).unwrap();
            let img = document.create_element("img")?;
            img.set_attribute("style", "width: 100%; height:auto")?;
            match (&board.state, board.at(&Point { x, y })) {
                (Ready, Some(Number { state: Flagged, .. }))
                | (Ready, Some(Mine { state: Flagged, .. }))
                | (Playing, Some(Number { state: Flagged, .. }))
                | (Playing, Some(Mine { state: Flagged, .. })) => {
                    img.set_attribute("src", "svg/flag.svg")?
                }
                (Ready, Some(Number { state: Closed, .. }))
                | (Ready, Some(Mine { state: Closed, .. }))
                | (Playing, Some(Number { state: Closed, .. }))
                | (Playing, Some(Mine { state: Closed, .. })) => {
                    img.set_attribute("src", "svg/question.svg")?
                }
                (_, Some(Number { count, .. })) => {
                    img.set_attribute("src", &format!("svg/{}.svg", *count))?
                }
                (Failed, Some(Mine { .. })) => img.set_attribute("src", "svg/bomb.svg")?,
                (Won, Some(Mine { .. })) => img.set_attribute("src", "svg/flag.svg")?,
                _ => unreachable!(),
            };
            inner_div.append_child(&img).unwrap();
        }
        let inner_div = document.create_element("div")?;
        inner_div.set_attribute("class", "break")?;
        div.append_child(&inner_div).unwrap();
    }
    board_game_placeholder.append_child(&div).unwrap();

    Ok(())
}

struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

enum Msg {
    ToggleDifficulty,
    ToggleMode,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct State {
    difficulty: Difficulty,
    mode: Mode,
    board: Board,
}

const KEY: &'static str = "jgpaiva.minesweeper.self";

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        //        let difficulty = {
        //            if let Json(Ok(restored_model)) = storage.restore(KEY) {
        //                restored_model
        //            } else {
        //
        //            }
        //        };
        let state = State {
            difficulty: Difficulty::Easy,
            mode: Mode::Digging,
            board: small_board(),
        };
        Self {
            link,
            storage,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleDifficulty => self.toggle_difficulty(),
            Msg::ToggleMode => {
                self.state = match self.state.mode {
                    Mode::Digging => State {
                        mode: Mode::Flagging,
                        ..self.state.clone()
                    },
                    Mode::Flagging => State {
                        mode: Mode::Digging,
                        ..self.state.clone()
                    },
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <body>
                <div id="difficulty_button_placeholder" class="flex-container">
                <div
                    id="difficulty-button"
                    class="clickable item"
                    onclick=self.link.callback(|_| Msg::ToggleDifficulty) >
                    { self.view_difficulty() }
                    </div>
                </div>
                <div id="mode_button_placeholder" class="flex-container">
                    <div
                        id="mode-button"
                        class="clickable item"
                        onclick=self.link.callback(|_| Msg::ToggleMode) >
                        <img class="svg_container" src={ self.view_mode() } />
                    </div>
                </div>

                <div id="board_game_placeholder">
                    <div id="board_game" class="flex-container">
                        {
                            (0..self.state.board.height)
                                .flat_map(|y| {
                                                (0..self.state.board.width+1).map(move |x| {
                                                    if x == self.state.board.width{
                                                        self.view_break()
                                                    } else {
                                                        self.view_item(x,y)
                                                    }
                                                })
                                }).collect::<Html>()
                        }
                    </div>
                </div>
            </body>
        }
    }
}

impl Model {
    fn toggle_difficulty(&mut self) {
        let (new_board, new_difficulty) = match (
            self.state.board.state.clone(),
            self.state.difficulty.clone(),
        ) {
            (Ready, Difficulty::Easy) => (medium_board(), Difficulty::Medium),
            (Ready, Difficulty::Medium) => (large_board(), Difficulty::Hard),
            (Ready, Difficulty::Hard) => (small_board(), Difficulty::Easy),
            (_, Difficulty::Easy) => (small_board(), Difficulty::Easy),
            (_, Difficulty::Medium) => (medium_board(), Difficulty::Medium),
            (_, Difficulty::Hard) => (large_board(), Difficulty::Hard),
        };
        self.state = State {
            difficulty: new_difficulty,
            board: new_board,
            ..self.state.clone()
        }
    }

    fn view_difficulty(&self) -> Html {
        html! {
            match self.state.difficulty {
                Difficulty::Easy => "😀",
                Difficulty::Medium => "🤨",
                Difficulty::Hard => "🧐",
            }
        }
    }

    fn view_mode(&self) -> &str {
        match self.state.mode {
            Mode::Flagging => "svg/flag.svg",
            Mode::Digging => "svg/dig.svg",
        }
    }

    fn view_item(&self, x: usize, y: usize) -> Html {
        let x = x as i32;
        let y = y as i32;
        let p = Point { x, y };
        html! {
            <div class="item active", style={self.get_item_style()}>
                <img style="width:100%" src={
                    match (self.state.board.state.clone(), self.state.board.at(&Point { x, y })) {
                        (Ready, Some(Number { state: Flagged, .. }))
                            | (Ready, Some(Mine { state: Flagged, .. }))
                            | (Playing, Some(Number { state: Flagged, .. }))
                            | (Playing, Some(Mine { state: Flagged, .. })) => {
                                String::from("svg/flag.svg")
                            }
                        (Ready, Some(Number { state: Closed, .. }))
                            | (Ready, Some(Mine { state: Closed, .. }))
                            | (Playing, Some(Number { state: Closed, .. }))
                            | (Playing, Some(Mine { state: Closed, .. })) => {
                                String::from("svg/question.svg")
                            }
                        (_, Some(Number { count, .. })) => {
                            format!("svg/{}.svg", *count)
                        }
                        (Failed, Some(Mine { .. })) => String::from("svg/bomb.svg"),
                        (Won, Some(Mine { .. })) => String::from("svg/flag.svg"),
                        _ => unreachable!(),
                    }
                }/>
            </div>
        }
    }

    fn get_item_style(&self) -> String {
        let square_size: f64 = 100.0 / (self.state.board.width as f64);
        let margin: f64 = 0.05 * square_size;
        let width = format!("{:.2}", square_size - 2.0 * margin);

        format!("width: {}%; margin: {}%", width, margin)
    }

    fn view_break(&self) -> Html {
        html! {
            <div class="break">
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    create_structure()?;
    render_page()
    //yew::initialize();
    //App::<Model>::new().mount_as_body();
    //Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_create_item_asymetric_board() {
        let square = create_item(20, 10);
        let mut props = HashMap::new();
        props.insert("class".to_string(), "item".to_string());
        props.insert(
            "style".to_string(),
            "width: 4.50%; margin: 0.25%".to_string(),
        );
        let expected_item = CellItem { props };

        assert_eq!(square, expected_item);
    }

    #[test]
    fn test_create_item_asymetric_reversed_board() {
        let square = create_item(10, 20);
        let mut props = HashMap::new();
        props.insert("class".to_string(), "item".to_string());
        props.insert(
            "style".to_string(),
            "width: 9.00%; margin: 0.5%".to_string(),
        );
        let expected_item = CellItem { props };

        assert_eq!(square, expected_item);
    }

    #[test]
    fn test_create_item_smaller_board() {
        let square = create_item(10, 10);
        let mut props = HashMap::new();
        props.insert("class".to_string(), "item".to_string());
        props.insert(
            "style".to_string(),
            "width: 9.00%; margin: 0.5%".to_string(),
        );
        let expected_item = CellItem { props };

        assert_eq!(square, expected_item);
    }
}
