use std::collections::HashMap;

use lib_minesweeper::create_board;
use lib_minesweeper::numbers_on_board;
use lib_minesweeper::Board;
use lib_minesweeper::BoardState;
use lib_minesweeper::MapElement::Mine;
use lib_minesweeper::MapElement::Number;
use lib_minesweeper::MapElementCellState::Closed;
use lib_minesweeper::Point;

#[cfg(test)]
pub mod tests2 {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_create_item() {
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
    fn test_create_item2() {
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

#[derive(Debug, PartialEq)]
struct SvgSquare {
    props: HashMap<String, String>,
}

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

#[derive(Debug, PartialEq)]
struct CellItem {
    props: HashMap<String, String>,
}

fn create_item(width: usize, height: usize) -> CellItem {
    let mut props = HashMap::new();
    let square_size: f64 = 100.0 / (height.max(width) as f64);
    let margin: f64 = 0.05 * square_size;
    let width = format!("{:.2}", square_size - 2.0 * margin).to_string();

    let style = format!("width: {}%; margin: {}%", width, margin);
    props.insert("style".to_string(), style.to_string());
    props.insert("class".to_string(), "item".to_string());
    CellItem { props }
}

#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref BOARD: Mutex<Board> = Mutex::new(small_board());
}

pub fn create_board_page(board: &Board) -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    match board.state {
        BoardState::Playing => body.set_attribute("class", "ongoing"),
        BoardState::Won => body.set_attribute("class", "won"),
        BoardState::Failed => body.set_attribute("class", "failed"),
        BoardState::NotReady => unreachable!(),
    }?;

    let div = document.create_element("div")?;
    div.set_attribute("class", "flex-container")?;
    div.set_attribute("id", "board_game")?;

    for y in 0..board.height {
        for x in 0..board.width {
            let x = x as i32;
            let y = y as i32;
            let inner_div = document.create_element("div")?;
            let is_done = matches!(board.state, BoardState::Failed | BoardState::Won);
            if !is_done {
                let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                    let window = web_sys::window().expect("no global `window` exists");
                    let document = window.document().expect("should have a document on window");
                    let div = document.get_element_by_id("board_game").unwrap();
                    let body = div.parent_node().unwrap();
                    body.remove_child(&div);
                    let mut board = BOARD.lock().unwrap();
                    let val = board.cascade_open_item(&Point { x, y });
                    if val.is_some() {
                        *board = val.unwrap();
                    }

                    create_board_page(&board);
                }) as Box<dyn FnMut(_)>);
                inner_div
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
                closure.forget();
            }
            let item = create_item(board.width, board.height);
            for (k, v) in item.props {
                inner_div.set_attribute(&k, &v)?;
            }
            if is_done {
                inner_div.set_attribute("class", "item frozen")?;
            } else {
                inner_div.set_attribute("class", "item active")?;
            }
            div.append_child(&inner_div).unwrap();
            let img = document.create_element("img")?;
            img.set_attribute("style", "width: 100%; height:auto")?;
            match (is_done, board.at(&Point { x, y })) {
                (false, Some(Number { state: Closed, .. }))
                | (false, Some(Mine { state: Closed, .. })) => {
                    img.set_attribute("src", "svg/question.svg")?
                }
                (_, Some(Number { count, .. })) => {
                    img.set_attribute("src", &format!("svg/{}.svg", *count))?
                }
                (_, Some(Mine { .. })) => img.set_attribute("src", "svg/bomb.svg")?,
                _ => unreachable!(),
            };
            inner_div.append_child(&img).unwrap();
        }
        let inner_div = document.create_element("div")?;
        inner_div.set_attribute("class", "break")?;
        div.append_child(&inner_div).unwrap();
    }
    body.append_child(&div).unwrap();

    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    create_board_page(&*BOARD.lock().unwrap())
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
