use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum MapElement {
    Mine {
        state: MapElementCellState,
    },
    Number {
        state: MapElementCellState,
        count: i32,
    },
}
#[derive(Debug, PartialEq, Clone)]
pub enum MapElementCellState {
    Closed,
    Open,
    Flagged,
}

use MapElement::Mine;
use MapElement::Number;
use MapElementCellState::Closed;
use MapElementCellState::Flagged;
use MapElementCellState::Open;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        let x = x as i32;
        let y = y as i32;
        Point { x, y }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BoardState {
    NotReady,
    Playing,
    Won,
    Failed,
}
pub struct Board {
    map: Vec<Vec<MapElement>>,
    missing_points: i32,
    pub width: usize,
    pub height: usize,
    pub mines: usize,
    pub state: BoardState,
}

impl Board {
    pub fn new(map: Vec<Vec<MapElement>>) -> Board {
        let mines = map
            .iter()
            .flat_map(|x| x.iter())
            .filter(|x| matches!(x, Mine{..}))
            .count();
        let width = map.iter().next().unwrap().len();
        let height = map.len();
        Board {
            width,
            height,
            mines,
            missing_points: (width as i32) * (height as i32) - (mines as i32),
            state: BoardState::NotReady,
            map,
        }
    }

    pub fn at(self: &Self, p: &Point) -> Option<&MapElement> {
        let width = self.width as i32;
        let height = self.height as i32;
        if p.x < 0 || p.x >= width || p.y < 0 || p.y >= height {
            None
        } else {
            let x = p.x as usize;
            let y = p.y as usize;
            Some(&self.map[y][x])
        }
    }

    fn replace(self: &Self, p: &Point, el: MapElement) -> Board {
        let was_closed = matches!(self.at(p), Some(Number { state: Closed, .. }));
        let map = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| {
                        if Point::new(x, y) == *p {
                            el.clone()
                        } else {
                            self.at(&Point::new(x, y)).unwrap().clone()
                        }
                    })
                    .collect()
            })
            .collect();
        let missing_points = if was_closed {
            self.missing_points - 1
        } else {
            self.missing_points
        };
        Board {
            width: self.width,
            height: self.height,
            mines: self.mines,
            missing_points,
            map,
            state: if missing_points == 0 {
                BoardState::Won
            } else {
                self.state.clone()
            },
        }
    }

    pub fn flag_item(self: &Self, p: &Point) -> Board {
        match self.at(p) {
            Some(Mine { state }) => self.replace(
                p,
                Mine {
                    state: match *state {
                        Closed => Flagged,
                        Flagged => Closed,
                        Open => Open,
                    },
                },
            ),
            Some(Number { state, count }) => self.replace(
                p,
                Number {
                    state: match *state {
                        Closed => Flagged,
                        Flagged => Closed,
                        Open => Open,
                    },
                    count: *count,
                },
            ),
            None => unreachable!(),
        }
    }

    pub fn cascade_open_item(self: &Self, p: &Point) -> Option<Board> {
        match self.at(p).unwrap() {
            Number { state: Open, .. }
            | Mine { state: Flagged, .. }
            | Number { state: Flagged, .. } => None,
            Number {
                state: Closed,
                count,
            } => {
                let board = self.replace(
                    p,
                    Number {
                        state: Open,
                        count: *count,
                    },
                );
                if *count == 0 {
                    Some(
                        board
                            .surrounding_points(&p)
                            .iter()
                            .fold(board, |b: Board, p| b.cascade_open_item(&p).unwrap_or(b)),
                    )
                } else {
                    Some(board)
                }
            }
            Mine { state: Open } | Mine { state: Closed } => Some(Board {
                map: self.map.clone(),
                width: self.width,
                height: self.height,
                mines: self.mines,
                missing_points: self.missing_points,
                state: BoardState::Failed,
            }),
        }
    }

    pub fn surrounding_points(self: &Self, p: &Point) -> Vec<Point> {
        [p.x - 1, p.x, p.x + 1]
            .iter()
            .flat_map(|&x| {
                [p.y - 1, p.y, p.y + 1]
                    .iter()
                    .map(|&y| Point { x, y })
                    .filter(|&Point { x, y }| p.x != x || p.y != y)
                    .filter(|p| self.at(p).is_some())
                    .collect::<Vec<Point>>()
            })
            .collect()
    }
}

pub fn create_board(
    width: usize,
    height: usize,
    mines: usize,
    mut rand: impl FnMut(usize, usize) -> usize,
) -> Board {
    let mut points: Vec<Point> = Vec::with_capacity(mines);
    for _ in 0..mines {
        loop {
            let x = rand(0, width);
            let y = rand(0, height);
            let p = Point::new(x, y);
            if points.contains(&p) {
                continue;
            }
            points.push(p);
            break;
        }
    }

    let map = (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    if points.contains(&Point::new(x, y)) {
                        Mine { state: Closed }
                    } else {
                        Number {
                            state: Closed,
                            count: 0,
                        }
                    }
                })
                .collect()
        })
        .collect();
    Board::new(map)
}

pub fn numbers_on_board(board: Board) -> Board {
    let map = (0..board.height)
        .map(|y| {
            (0..board.width)
                .map(|x| {
                    let point = Point::new(x, y);
                    match board.at(&point).unwrap() {
                        Mine { state } => Mine {
                            state: state.clone(),
                        },
                        Number { count: 0, state } => {
                            let count = board
                                .surrounding_points(&point)
                                .iter()
                                .filter(|p| matches!(board.at(p), Some(Mine { .. })))
                                .count() as i32;
                            Number {
                                state: state.clone(),
                                count,
                            }
                        }
                        _ => unreachable!(),
                    }
                })
                .collect()
        })
        .collect();
    Board {
        map,
        state: BoardState::Playing,
        ..board
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn state_from_bytes(state: u8) -> MapElementCellState {
        match state {
            b'O' => Open,
            b'C' => Closed,
            b'F' => Flagged,
            _ => unreachable!(),
        }
    }

    fn count_from_bytes(c: u8) -> i32 {
        (c as i32) - (b'0' as i32)
    }

    fn make_map(map: Vec<String>, state: Vec<String>) -> Vec<Vec<MapElement>> {
        map.iter()
            .zip(state)
            .map(|(map_row, state_row)| {
                map_row
                    .as_bytes()
                    .iter()
                    .zip(state_row.as_bytes())
                    .map(|(row_el, state_el)| match row_el {
                        b'X' => Mine {
                            state: state_from_bytes(*state_el),
                        },
                        _ => Number {
                            state: state_from_bytes(*state_el),
                            count: count_from_bytes(*row_el),
                        },
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test_make_map() {
        let map = make_map(
            vec![String::from("00"), String::from("22"), String::from("XX")],
            vec![String::from("OC"), String::from("FC"), String::from("CF")],
        );
        let expected_map = vec![
            vec![
                Number {
                    count: 0,
                    state: Open,
                },
                Number {
                    count: 0,
                    state: Closed,
                },
            ],
            vec![
                Number {
                    count: 2,
                    state: Flagged,
                },
                Number {
                    count: 2,
                    state: Closed,
                },
            ],
            vec![Mine { state: Closed }, Mine { state: Flagged }],
        ];

        assert_eq!(map, expected_map);
    }

    pub fn five_by_four_board() -> Board {
        Board::new(make_map(
            vec![
                String::from("X0000"),
                String::from("0X000"),
                String::from("00X00"),
                String::from("000X0"),
            ],
            vec![
                String::from("CCCCC"),
                String::from("CCCCC"),
                String::from("CCCCC"),
                String::from("CCCCC"),
            ],
        ))
    }

    pub fn five_by_two_board() -> Board {
        Board::new(make_map(
            vec![String::from("X0000"), String::from("0X000")],
            vec![String::from("CCCCC"), String::from("CCCCC")],
        ))
    }

    #[test]
    fn test_create_board() {
        let width = 5;
        let height = 4;
        let mines = 4;
        let mut v = vec![3, 3, 2, 2, 1, 1, 0, 0];
        let rand = move |_start: usize, _end: usize| -> usize {
            return v.pop().unwrap();
        };
        let board = create_board(width, height, mines, rand);
        let expected_map = five_by_four_board().map;
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::NotReady);
    }

    #[test]
    fn test_create_board_without_repeated_mines() {
        let width = 5;
        let height = 4;
        let mines = 4;
        let mut v = vec![3, 3, 2, 2, 0, 0, 1, 1, 0, 0];
        let rand = move |_start: usize, _end: usize| -> usize {
            return v.pop().unwrap();
        };
        let board = create_board(width, height, mines, rand);
        let expected_map = five_by_four_board().map;
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::NotReady);
    }

    #[test]
    fn test_numbers_on_board() {
        let board = numbers_on_board(five_by_four_board());
        let expected_map = make_map(
            vec![
                String::from("X2100"),
                String::from("2X210"),
                String::from("12X21"),
                String::from("012X1"),
            ],
            vec![
                String::from("CCCCC"),
                String::from("CCCCC"),
                String::from("CCCCC"),
                String::from("CCCCC"),
            ],
        );
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Playing);
    }

    #[test]
    fn test_surrounding_points() {
        assert_eq!(
            five_by_two_board().surrounding_points(&Point { x: 1, y: 0 }),
            vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 0 },
                Point { x: 2, y: 1 },
            ]
        );
    }

    #[test]
    fn test_cascade_open_item() {
        let board = numbers_on_board(five_by_two_board());
        let board = board.cascade_open_item(&Point::new(3, 1)).unwrap();
        let expected_map = make_map(
            vec![String::from("X2100"), String::from("2X100")],
            vec![String::from("CCOOO"), String::from("CCOOO")],
        );
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Playing);
    }

    #[test]
    fn test_win_board() {
        let board = numbers_on_board(five_by_two_board());
        let board = board.cascade_open_item(&Point::new(3, 1)).unwrap();
        let board = board.cascade_open_item(&Point::new(0, 1)).unwrap();
        let board = board.cascade_open_item(&Point::new(1, 0)).unwrap();
        let expected_map = make_map(
            vec![String::from("X2100"), String::from("2X100")],
            vec![String::from("COOOO"), String::from("OCOOO")],
        );
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Won);
    }

    #[test]
    fn test_flag() {
        let board = numbers_on_board(five_by_two_board());
        let board = board.flag_item(&Point::new(3, 1));
        let expected_map = make_map(
            vec![String::from("X2100"), String::from("2X100")],
            vec![String::from("CCCCC"), String::from("CCCFC")],
        );
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Playing);
    }

    #[test]
    fn test_flagging_again_unflags() {
        let board = numbers_on_board(five_by_two_board());
        let board = board.flag_item(&Point::new(3, 1));
        let board = board.flag_item(&Point::new(3, 1));
        let expected_map = make_map(
            vec![String::from("X2100"), String::from("2X100")],
            vec![String::from("CCCCC"), String::from("CCCCC")],
        );
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Playing);
    }

    #[test]
    fn test_flagging_open_does_noting() {
        let board = numbers_on_board(five_by_two_board());
        let board = board.cascade_open_item(&Point::new(2, 0)).unwrap();
        let board = board.flag_item(&Point::new(2, 0));
        let expected_map = make_map(
            vec![String::from("X2100"), String::from("2X100")],
            vec![String::from("CCOCC"), String::from("CCCCC")],
        );
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Playing);
    }
}

#[cfg(test)]
pub mod tests2 {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_create_square() {
        let square = create_square(20, 10, 1, 0);
        let mut props = HashMap::new();
        props.insert("width".to_string(), "4.00%".to_string());
        props.insert("height".to_string(), "4.00%".to_string());
        props.insert("x".to_string(), "5.50%".to_string());
        props.insert("y".to_string(), "0.50%".to_string());
        props.insert("fill".to_string(), "grey".to_string());
        let expected_square = SvgSquare { props };

        assert_eq!(square, expected_square);
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

fn create_square(width: usize, height: usize, x: i32, y: i32) -> SvgSquare {
    let mut props = HashMap::new();
    let padding: f64 = 1.0;
    let square_size: f64 = 100.0 / (height.max(width) as f64);
    let width = format!("{:.2}%", square_size - padding).to_string();
    let height = format!("{:.2}%", square_size - padding).to_string();
    let x = format!("{:.2}%", square_size * (x as f64) + (padding / 2.0)).to_string();
    let y = format!("{:.2}%", square_size * (y as f64) + (padding / 2.0)).to_string();

    props.insert("width".to_string(), width);
    props.insert("height".to_string(), height);
    props.insert("x".to_string(), x);
    props.insert("y".to_string(), y);
    props.insert("fill".to_string(), "grey".to_string());
    SvgSquare { props }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let mut board = small_board();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let div = document
        .create_element("div")
        .unwrap()
        .dyn_into::<web_sys::HtmlDivElement>()
        .unwrap();
    div.set_attribute("class", "flex-container")?;

    for y in 0..board.height {
        for x in 0..board.width {
            let x = x as i32;
            let y = y as i32;
            let square = create_square(board.height, board.width, x, y);
            let inner_div = document.create_element("div")?;
            inner_div.set_attribute("class", "item");
            inner_div.set_attribute(
                "style",
                &format!(
                    "width:{:.2}%; margin:1%",
                    100.0 / (board.width as f64) - 2.0
                ),
            );
            inner_div.set_inner_html(&format!("{} {}", x, y));
            div.append_child(&inner_div).unwrap();
        }
        let inner_div = document.create_element("div")?;
        inner_div.set_attribute("class", "break");
        div.append_child(&inner_div).unwrap();
    }
    body.append_child(&div).unwrap();

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
