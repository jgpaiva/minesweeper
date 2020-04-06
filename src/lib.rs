#[derive(Debug, PartialEq)]
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

pub struct Board {
    map: Vec<Vec<MapElement>>,
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

impl Board {
    pub fn at(self: &Self, p: &Point) -> Option<&MapElement> {
        let x = p.x;
        let y = p.y;
        let width = self.width as i32;
        let height = self.height as i32;
        if x < 0 || x >= width || y < 0 || y >= height {
            return None;
        } else {
            let x = x as usize;
            let y = y as usize;
            return Some(&self.map[y][x]);
        }
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
                .map(|x| match points.contains(&Point::new(x, y)) {
                    true => MapElement::Mine { open: false },
                    false => MapElement::Empty { open: false },
                })
                .collect()
        })
        .collect();
    Board {
        map,
        width,
        height,
        mines,
    }
}

pub fn numbers_on_board(board: Board) -> Board {
    let map = (0..board.height)
        .map(|y| {
            (0..board.width)
                .map(|x| match board.at(&Point::new(x, y)) {
                    Some(MapElement::Mine { open: _ }) => MapElement::Mine { open: false },
                    Some(MapElement::Empty { open: _ }) => {
                        let x = x as i32;
                        let y = y as i32;
                        let count: i32 = [
                            Point { x: x - 1, y: y - 1 },
                            Point { x: x, y: y - 1 },
                            Point { x: x + 1, y: y - 1 },
                            Point { x: x - 1, y: y + 1 },
                            Point { x: x, y: y + 1 },
                            Point { x: x + 1, y: y + 1 },
                            Point { x: x - 1, y: y },
                            Point { x: x + 1, y: y },
                        ]
                        .iter()
                        .map(|p| match board.at(p) {
                            None => 0,
                            Some(MapElement::Mine { open: _ }) => 1,
                            Some(MapElement::Empty { open: _ }) => 0,
                            _ => 0,
                        })
                        .sum();
                        match count {
                            0 => MapElement::Empty { open: false },
                            _ => MapElement::Number { open: false, count },
                        }
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    Board {
        height: board.height,
        width: board.width,
        mines: board.mines,
        map: map,
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
    }

    #[test]
    fn test_numbers_on_board() {
        let board = Board {
            height: 4,
            width: 5,
            mines: 4,
            map: vec![
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
        };
        let board_with_numbers = numbers_on_board(board);
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
        assert_eq!(board_with_numbers.map, expected_map);
    }
}
