#[derive(Debug, PartialEq, Clone)]
pub enum MapElement {
    Mine { open: bool },
    Empty { open: bool },
    Number { open: bool, count: i32 },
}

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
    pub fn new(width: usize, height: usize, mines: usize, map: Vec<Vec<MapElement>>) -> Board {
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

    pub fn replace(self: &Self, p: &Point, el: MapElement) -> Board {
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
        Board {
            width: self.width,
            height: self.height,
            mines: self.mines,
            missing_points: self.missing_points,
            map,
            state: self.state.clone(),
        }
    }

    pub fn open_item(self: &Self, p: &Point) -> Board {
        let board_point = self.at(p);

        let newpoint = match board_point {
            Some(MapElement::Empty { open: false }) => Some(MapElement::Empty { open: true }),
            Some(MapElement::Number { open: false, count }) => Some(MapElement::Number {
                open: true,
                count: *count,
            }),
            _ => None,
        };

        match newpoint {
            Some(newpoint) => self.replace(p, newpoint),
            None => Board {
                map: self.map.clone(),
                width: self.width,
                height: self.height,
                mines: self.mines,
                missing_points: self.missing_points,
                state: BoardState::Failed,
            },
        }
    }

    pub fn cascade_open_item(self: &Self, p: &Point) -> Option<Board> {
        if matches!(self.at(p).unwrap(), MapElement::Mine{open:true} | MapElement::Empty{open:true} | MapElement::Number{open:true, ..})
        {
            return None;
        }

        let board = self.open_item(p);
        if matches!(board.state, BoardState::Failed) {
            return Some(board);
        }

        if matches!(board.at(&p).unwrap(), MapElement::Empty { open: true }) {
            return Some(
                board
                    .surrounding_points(&p)
                    .iter()
                    .fold(board, |b: Board, p| b.cascade_open_item(&p).unwrap_or(b)),
            );
        }

        Some(board)
    }

    pub fn surrounding_points(self: &Self, p: &Point) -> Vec<Point> {
        [p.x - 1, p.x, p.x + 1]
            .iter()
            .flat_map(|&x| {
                [p.y - 1, p.y, p.y + 1]
                    .iter()
                    .map(|&y| Point { x, y })
                    .filter(|&Point { x, y }| p.x != x || p.y != y)
                    .filter(|p| !matches!(self.at(p), None))
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
                        MapElement::Mine { open: false }
                    } else {
                        MapElement::Empty { open: false }
                    }
                })
                .collect()
        })
        .collect();
    Board::new(width, height, mines, map)
}

pub fn numbers_on_board(board: Board) -> Board {
    let map = (0..board.height)
        .map(|y| {
            (0..board.width)
                .map(|x| {
                    let point = Point::new(x, y);
                    match board.at(&point) {
                        Some(MapElement::Mine { .. }) => MapElement::Mine { open: false },
                        Some(MapElement::Empty { .. }) => {
                            let count = board
                                .surrounding_points(&point)
                                .iter()
                                .map(|p| match board.at(p) {
                                    None => 0,
                                    Some(MapElement::Mine { .. }) => 1,
                                    Some(MapElement::Empty { .. }) => 0,
                                    _ => 0,
                                })
                                .sum();
                            match count {
                                0 => MapElement::Empty { open: false },
                                _ => MapElement::Number { open: false, count },
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
mod tests {
    use super::*;

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
        let expected_map = vec![
            vec![
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
            ],
        ];
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
        let expected_map = vec![
            vec![
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
            ],
        ];
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::NotReady);
    }

    #[test]
    fn test_numbers_on_board() {
        let board = Board::new(
            5,
            4,
            4,
            vec![
                vec![
                    MapElement::Mine { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                ],
                vec![
                    MapElement::Empty { open: false },
                    MapElement::Mine { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                ],
                vec![
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Mine { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                ],
                vec![
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Mine { open: false },
                    MapElement::Empty { open: false },
                ],
            ],
        );
        let board = numbers_on_board(board);
        let expected_map = vec![
            vec![
                MapElement::Mine { open: false },
                MapElement::Number {
                    open: false,
                    count: 2,
                },
                MapElement::Number {
                    open: false,
                    count: 1,
                },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Number {
                    open: false,
                    count: 2,
                },
                MapElement::Mine { open: false },
                MapElement::Number {
                    open: false,
                    count: 2,
                },
                MapElement::Number {
                    open: false,
                    count: 1,
                },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Number {
                    open: false,
                    count: 1,
                },
                MapElement::Number {
                    open: false,
                    count: 2,
                },
                MapElement::Mine { open: false },
                MapElement::Number {
                    open: false,
                    count: 2,
                },
                MapElement::Number {
                    open: false,
                    count: 1,
                },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Number {
                    open: false,
                    count: 1,
                },
                MapElement::Number {
                    open: false,
                    count: 2,
                },
                MapElement::Mine { open: false },
                MapElement::Number {
                    open: false,
                    count: 1,
                },
            ],
        ];
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Playing);
    }

    fn two_by_five_board() -> Board {
        Board::new(
            5,
            2,
            4,
            vec![
                vec![
                    MapElement::Mine { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                ],
                vec![
                    MapElement::Empty { open: false },
                    MapElement::Mine { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                ],
            ],
        )
    }

    #[test]
    fn test_surrounding_points() {
        assert_eq!(
            two_by_five_board().surrounding_points(&Point { x: 1, y: 0 }),
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
    fn test_valid_open_item() {
        let board = numbers_on_board(two_by_five_board());
        let board = board.open_item(&Point::new(1, 0));
        let expected_map = vec![
            vec![
                MapElement::Mine { open: false },
                MapElement::Number {
                    count: 2,
                    open: true,
                },
                MapElement::Number {
                    count: 1,
                    open: false,
                },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Number {
                    count: 2,
                    open: false,
                },
                MapElement::Mine { open: false },
                MapElement::Number {
                    count: 1,
                    open: false,
                },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
        ];
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Playing);
    }

    #[test]
    fn test_invalid_open_item() {
        let board = numbers_on_board(two_by_five_board());
        let board = board.open_item(&Point::new(0, 0));
        let expected_map = vec![
            vec![
                MapElement::Mine { open: false },
                MapElement::Number {
                    count: 2,
                    open: false,
                },
                MapElement::Number {
                    count: 1,
                    open: false,
                },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Number {
                    count: 2,
                    open: false,
                },
                MapElement::Mine { open: false },
                MapElement::Number {
                    count: 1,
                    open: false,
                },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
        ];
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Failed);
    }

    #[test]
    fn test_cascade_open_item() {
        let board = numbers_on_board(two_by_five_board());
        let board = board.cascade_open_item(&Point::new(3, 1));
        let board = board.unwrap();
        let expected_map = vec![
            vec![
                MapElement::Mine { open: false },
                MapElement::Number {
                    count: 2,
                    open: false,
                },
                MapElement::Number {
                    count: 1,
                    open: true,
                },
                MapElement::Empty { open: true },
                MapElement::Empty { open: true },
            ],
            vec![
                MapElement::Number {
                    count: 2,
                    open: false,
                },
                MapElement::Mine { open: false },
                MapElement::Number {
                    count: 1,
                    open: true,
                },
                MapElement::Empty { open: true },
                MapElement::Empty { open: true },
            ],
        ];
        assert_eq!(board.map, expected_map);
        assert_eq!(board.state, BoardState::Playing);
    }
}
