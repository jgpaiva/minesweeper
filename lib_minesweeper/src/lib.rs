use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum MapElement {
    Mine {
        state: MapElementCellState,
    },
    Number {
        state: MapElementCellState,
        count: i32,
    },
}
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        let x = x as i32;
        let y = y as i32;
        Point { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum BoardState {
    NotReady,
    Ready,
    Playing,
    Won,
    Failed,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
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
            .filter(|x| matches!(x, Mine { .. }))
            .count();
        let width = map.first().unwrap().len();
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

    pub fn at(&self, p: &Point) -> Option<&MapElement> {
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

    fn replace(&self, p: &Point, el: MapElement) -> Board {
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
            state: match (missing_points, &self.state) {
                (0, _) => BoardState::Won,
                (_, BoardState::Ready) => BoardState::Playing,
                _ => self.state.clone(),
            },
        }
    }

    pub fn flag_item(&self, p: &Point) -> Board {
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

    pub fn cascade_open_item(&self, p: &Point) -> Option<Board> {
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
                            .surrounding_points(p)
                            .iter()
                            .fold(board, |b: Board, p| b.cascade_open_item(p).unwrap_or(b)),
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

    pub fn surrounding_points(&self, p: &Point) -> Vec<Point> {
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

    pub fn run_robot_on_point(&self, p: Point) -> Option<Board> {
        let el = self.at(&p).unwrap();
        let Number {
            state: Open,
            count: mine_count,
        } = el else {
            return None
        };
        if *mine_count == 0 {
            return None;
        }
        let surrounding_points = self.surrounding_points(&p);
        let surrounding_els: Vec<_> = surrounding_points
            .iter()
            .map(|p| (p, self.at(p).unwrap().clone()))
            .filter(|(_p, el)| {
                !matches!(
                    el,
                    Number {
                        state: Open,
                        count: 0
                    }
                )
            })
            .collect();
        let mut unopened = surrounding_els
            .iter()
            .filter(|(_p, el)| !matches!(el, Number { state: Open, .. }));
        let flagged_count = surrounding_els
            .iter()
            .filter(|(_p, el)| {
                matches!(el, Mine { state: Flagged } | Number { state: Flagged, .. })
            })
            .count();
        let unopened_count = unopened.clone().count();
        if *mine_count == unopened_count as i32 && flagged_count < unopened_count {
            let (p, _el) = unopened
                .find(|(_p, el)| {
                    !matches!(el, Mine { state: Flagged } | Number { state: Flagged, .. })
                })
                .unwrap();
            return Some(self.flag_item(p));
        }
        if *mine_count == flagged_count as i32 && unopened_count - flagged_count > 0 {
            let (p, _el) = unopened
                .find(|(_p, el)| {
                    !matches!(el, Mine { state: Flagged } | Number { state: Flagged, .. })
                })
                .unwrap();
            return self.cascade_open_item(p);
        }
        None
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
        state: BoardState::Ready,
        ..board
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! board_matches {
        ($value: expr, $expected: literal) => {
            let s = board_to_string(&$value);
            for (i, (line_value, line_expected)) in s.lines().zip($expected.lines()).enumerate() {
                let line_expected = line_expected.trim();
                assert_eq!(line_value, line_expected, "boards didn't match on line {i}");
            }
        };
    }

    macro_rules! map {
        ($bombs: literal, $state: literal) => {
            make_map($bombs, $state)
        };
    }

    impl From<&u8> for MapElementCellState {
        fn from(state: &u8) -> Self {
            match state {
                b'O' => Open,
                b'C' => Closed,
                b'F' => Flagged,
                _ => unreachable!(),
            }
        }
    }

    fn count_from_bytes(c: u8) -> i32 {
        (c as i32) - (b'0' as i32)
    }

    fn make_map(map: &str, state: &str) -> Vec<Vec<MapElement>> {
        map.lines()
            .zip(state.lines())
            .map(|(map_row, state_row)| {
                let map_row = map_row.trim();
                let state_row = state_row.trim();
                map_row
                    .as_bytes()
                    .iter()
                    .zip(state_row.as_bytes())
                    .map(|(row_el, state_el)| match row_el {
                        b'X' => Mine {
                            state: state_el.into(),
                        },
                        _ => Number {
                            state: state_el.into(),
                            count: count_from_bytes(*row_el),
                        },
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test_make_map() {
        let map = map!(
            "00
             22
             XX",
            "OC
             FC
             CF"
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
        Board::new(map!(
            "X0000
             0X000
             00X00
             000X0",
            "CCCCC
             CCCCC
             CCCCC
             CCCCC"
        ))
    }

    pub fn five_by_two_board() -> Board {
        Board::new(map!(
            "X0000
             0X000",
            "CCCCC
             CCCCC"
        ))
    }

    #[test]
    fn test_create_board() {
        let width = 5;
        let height = 4;
        let mines = 4;
        let mut v = vec![3, 3, 2, 2, 1, 1, 0, 0];
        let rand = move |_start: usize, _end: usize| -> usize { v.pop().unwrap() };
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
        let rand = move |_start: usize, _end: usize| -> usize { v.pop().unwrap() };
        let board = create_board(width, height, mines, rand);
        let expected_map = five_by_four_board().map;
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::NotReady);
    }

    #[test]
    fn test_numbers_on_board() {
        let board = numbers_on_board(five_by_four_board());
        let expected_map = map!(
            "X2100
             2X210
             12X21
             012X1",
            "CCCCC
             CCCCC
             CCCCC
             CCCCC"
        );
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Ready);
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
        let expected_map = map!(
            "X2100
             2X100",
            "CCOOO
             CCOOO"
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
        let expected_map = map!(
            "X2100
             2X100",
            "COOOO
             OCOOO"
        );
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Won);
    }

    #[test]
    fn test_flag() {
        let board = numbers_on_board(five_by_two_board());
        let board = board.flag_item(&Point::new(3, 1));
        let expected_map = map!(
            "X2100
             2X100",
            "CCCCC
             CCCFC"
        );
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Playing);
    }

    #[test]
    fn test_flagging_again_unflags() {
        let board = numbers_on_board(five_by_two_board());
        let board = board.flag_item(&Point::new(3, 1));
        let board = board.flag_item(&Point::new(3, 1));
        let expected_map = map!(
            "X2100
             2X100",
            "CCCCC
             CCCCC"
        );
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Playing);
    }

    #[test]
    fn test_flagging_open_does_noting() {
        let board = numbers_on_board(five_by_two_board());
        let board = board.cascade_open_item(&Point::new(2, 0)).unwrap();
        let board = board.flag_item(&Point::new(2, 0));
        let expected_map = map!(
            "X2100
             2X100",
            "CCOCC
             CCCCC"
        );
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Playing);
    }

    fn board_to_string(board: &Board) -> String {
        let mut ret = String::default();
        for y in 0..board.height {
            for x in 0..board.width {
                let p = Point::new(x, y);
                let el = board.at(&p).unwrap();
                let v = match el {
                    Mine {
                        state: MapElementCellState::Flagged,
                    }
                    | Number {
                        state: MapElementCellState::Flagged,
                        ..
                    } => "F".to_string(),
                    Number {
                        state: MapElementCellState::Closed,
                        ..
                    }
                    | Mine {
                        state: MapElementCellState::Closed,
                    } => "•".to_string(),
                    Number {
                        state: MapElementCellState::Open,
                        count: 0,
                    } => "_".to_string(),
                    Number {
                        state: MapElementCellState::Open,
                        count,
                    } => count.to_string(),
                    _ => unreachable!(),
                };
                ret.push_str(&v);
            }
            ret.push('\n');
        }
        ret
    }

    #[test]
    fn test_run_robot_on_point() {
        let board = numbers_on_board(five_by_two_board());
        let res = board.run_robot_on_point(Point::new(4, 0));
        assert!(res.is_none());

        let board = board.flag_item(&Point::new(1, 1));
        let board = board.cascade_open_item(&Point::new(2, 0)).unwrap();
        board_matches!(
            board,
            "••1••
             •F•••"
        );
        let board = board.run_robot_on_point(Point::new(2, 0)).unwrap();
        board_matches!(
            board,
            "•21••
             •F•••"
        );
        let board = board.run_robot_on_point(Point::new(2, 0)).unwrap();
        board_matches!(
            board,
            "•21••
             •F1••"
        );
        let board = board.run_robot_on_point(Point::new(2, 0)).unwrap();
        board_matches!(
            board,
            "•21__
             •F1__"
        );
        let res = board.run_robot_on_point(Point::new(2, 0));
        assert!(res.is_none());
    }

    #[ignore] // currently working on this test
    #[test]
    fn test_advanced_run_robot_on_point() {
        let board = numbers_on_board(Board::new(map!(
            "X0X
             000
             000",
            "COC
             COO
             CCC"
        )));
        board_matches!(
            board,
            "•2•
             •21
             •••"
        );
        let board = board.run_robot_on_point(Point::new(4, 0)).unwrap();
        board_matches!(
            board,
            "•2F
             •21
             •••"
        );
    }
}
