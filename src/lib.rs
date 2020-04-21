#![recursion_limit = "512"]

use lib_minesweeper::create_board;
use lib_minesweeper::numbers_on_board;
use lib_minesweeper::Board;
use lib_minesweeper::BoardState;
use lib_minesweeper::BoardState::Failed;
use lib_minesweeper::BoardState::NotReady;
use lib_minesweeper::BoardState::Playing;
use lib_minesweeper::BoardState::Ready;
use lib_minesweeper::BoardState::Won;
use lib_minesweeper::MapElement;
use lib_minesweeper::MapElement::Mine;
use lib_minesweeper::MapElement::Number;
use lib_minesweeper::MapElementCellState::Closed;
use lib_minesweeper::MapElementCellState::Flagged;
use lib_minesweeper::Point;

use wasm_bindgen::prelude::*;

use serde_derive::{Deserialize, Serialize};
//use yew::format::Json;
use yew::prelude::*;
//use yew::services::storage::{Area, StorageService};

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
struct Model {
    link: ComponentLink<Self>,
    //storage: StorageService,
    state: State,
}

enum Msg {
    ToggleDifficulty,
    ToggleMode,
    UpdateBoard { point: Point },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct State {
    difficulty: Difficulty,
    mode: Mode,
    board: Board,
}

//const KEY: &'static str = "jgpaiva.minesweeper.self";

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        //let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
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
            //storage,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleDifficulty => self.toggle_difficulty(),
            Msg::ToggleMode => self.toggle_mode(),
            Msg::UpdateBoard { point } => self.update_board(point),
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <body class={self.view_body_class()}>
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
                        class={self.view_mode_class()}
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
                                                        let board = &self.state.board;
                                                        html!{
                                                            <BoardItem
                                                                x={x}
                                                                y={y}
                                                                board_state={board.state.clone()}
                                                                board_width={board.width}
                                                                element={board.at(&Point::new(x,y)).unwrap()}
                                                                update_signal={self.link.callback(|msg:Msg| msg)}/>
                                                        }
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
    fn toggle_mode(&mut self) {
        if matches!(self.state.board.state, Won | Failed) {
            return;
        }
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

    fn view_body_class(&self) -> &str {
        match self.state.board.state {
            Ready | Playing => "ongoing",
            Won => "won",
            Failed => "failed",
            NotReady => unreachable!(),
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

    fn view_mode_class(&self) -> &str {
        match &self.state.board.state {
            Won | Failed => "item",
            _ => "clickable item",
        }
    }

    fn view_mode(&self) -> &str {
        match (&self.state.board.state, self.state.mode.clone()) {
            (Ready, Mode::Flagging) | (Playing, Mode::Flagging) => "svg/flag.svg",
            (Ready, Mode::Digging) | (Playing, Mode::Digging) => "svg/dig.svg",
            (Won, _) => "svg/trophy.svg",
            (Failed, _) => "svg/skull.svg",
            _ => unreachable!(),
        }
    }

    fn view_break(&self) -> Html {
        html! {
            <div class="break">
            </div>
        }
    }

    fn update_board(&mut self, p: Point) {
        match self.state.mode {
            Mode::Digging => {
                let new_board = self.state.board.cascade_open_item(&p);
                if let Some(b) = new_board {
                    self.state.board = b
                }
            }
            Mode::Flagging => {
                self.state.board = self.state.board.flag_item(&p);
            }
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
struct BoardItemProps {
    x: usize,
    y: usize,
    board_state: BoardState,
    board_width: usize,
    element: MapElement,
    update_signal: Callback<Msg>,
}

struct BoardItem {
    link: ComponentLink<Self>,
    props: BoardItemProps,
}

impl Component for BoardItem {
    type Message = Msg;
    type Properties = BoardItemProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.x == props.x
            && self.props.y == props.y
            && self.props.board_state == props.board_state
            && self.props.board_width == props.board_width
            && self.props.element == props.element
        {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateBoard { point } => self.props.update_signal.emit(Msg::UpdateBoard { point }),
            _ => unreachable!(),
        }
        true
    }

    fn view(&self) -> Html {
        let x = self.props.x;
        let y = self.props.y;
        html! {
            <div
                class="item active",
                style={self.get_item_style()}
                onclick=self.link.callback(move |_| {Msg::UpdateBoard {point:Point::new(x,y)}}) >
                <img style="width:100%" src={
                    match (&self.props.board_state, &self.props.element) {
                        (Ready, Number { state: Flagged, .. })
                            | (Ready, Mine { state: Flagged, .. })
                            | (Playing, Number { state: Flagged, .. })
                            | (Playing, Mine { state: Flagged, .. }) => {
                                String::from("svg/flag.svg")
                            }
                        (Ready, Number { state: Closed, .. })
                            | (Ready, Mine { state: Closed, .. })
                            | (Playing, Number { state: Closed, .. })
                            | (Playing, Mine { state: Closed, .. }) => {
                                String::from("svg/question.svg")
                            }
                        (_, Number { count, .. }) => {
                            format!("svg/{}.svg", count)
                        }
                        (Failed, Mine { .. }) => String::from("svg/bomb.svg"),
                        (Won, Mine { .. }) => String::from("svg/flag.svg"),
                        _ => unreachable!(),
                    }
                }/>
            </div>
        }
    }
}

impl BoardItem {
    fn get_item_style(&self) -> String {
        let square_size: f64 = 100.0 / (self.props.board_width as f64);
        let margin: f64 = 0.05 * square_size;
        let width = format!("{:.2}", square_size - 2.0 * margin);

        format!("width: {}%; margin: {}%", width, margin)
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    yew::initialize();
    App::<Model>::new().mount_as_body();
    Ok(())
}
