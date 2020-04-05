#[derive(Debug, PartialEq)]
pub enum MapElement {
    Mine,
    Empty,
    Number { count: i32 },
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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
            let x = rand(0, width) as i32;
            let y = rand(0, height) as i32;
            let p = Point { x, y };
            if points.contains(&p) {
                continue;
            }
            points.push(p);
            break;
        }
    }

    let mut map: Vec<Vec<MapElement>> = Vec::with_capacity(height);

    for y in 0..height {
        let y = y as i32;
        let mut line: Vec<MapElement> = Vec::with_capacity(width);
        for x in 0..width {
            let x = x as i32;
            if points.contains(&Point { x, y }) {
                line.push(MapElement::Mine);
            } else {
                line.push(MapElement::Empty);
            }
        }
        map.push(line);
    }
    Board {
        map,
        width,
        height,
        mines,
    }
}

pub fn numbers_on_board(board: Board) -> Board {
    let mut map: Vec<Vec<MapElement>> = Vec::with_capacity(board.height);
    for y in 0..board.height {
        let mut line: Vec<MapElement> = Vec::with_capacity(board.width);
        let y = y as i32;
        for x in 0..board.width {
            let x = x as i32;
            let p = Point { x, y };
            let final_val = match board.at(&p) {
                Some(MapElement::Mine) => MapElement::Mine,
                Some(MapElement::Empty) => {
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
                        Some(MapElement::Mine) => 1,
                        Some(MapElement::Empty) => 0,
                        _ => 0,
                    })
                    .sum();
                    match count {
                        0 => MapElement::Empty,
                        _ => MapElement::Number { count },
                    }
                }
                _ => unreachable!(),
            };
            line.push(final_val);
        }
        map.push(line);
    }
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
                MapElement::Mine,
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Empty,
            ],
            vec![
                MapElement::Empty,
                MapElement::Mine,
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Empty,
            ],
            vec![
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Mine,
                MapElement::Empty,
                MapElement::Empty,
            ],
            vec![
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Mine,
                MapElement::Empty,
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
                MapElement::Mine,
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Empty,
            ],
            vec![
                MapElement::Empty,
                MapElement::Mine,
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Empty,
            ],
            vec![
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Mine,
                MapElement::Empty,
                MapElement::Empty,
            ],
            vec![
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Mine,
                MapElement::Empty,
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
                    MapElement::Mine,
                    MapElement::Empty,
                    MapElement::Empty,
                    MapElement::Empty,
                    MapElement::Empty,
                ],
                vec![
                    MapElement::Empty,
                    MapElement::Mine,
                    MapElement::Empty,
                    MapElement::Empty,
                    MapElement::Empty,
                ],
                vec![
                    MapElement::Empty,
                    MapElement::Empty,
                    MapElement::Mine,
                    MapElement::Empty,
                    MapElement::Empty,
                ],
                vec![
                    MapElement::Empty,
                    MapElement::Empty,
                    MapElement::Empty,
                    MapElement::Mine,
                    MapElement::Empty,
                ],
            ],
        };
        let board_with_numbers = numbers_on_board(board);
        let expected_map = vec![
            vec![
                MapElement::Mine,
                MapElement::Number { count: 2 },
                MapElement::Number { count: 1 },
                MapElement::Empty,
                MapElement::Empty,
            ],
            vec![
                MapElement::Number { count: 2 },
                MapElement::Mine,
                MapElement::Number { count: 2 },
                MapElement::Number { count: 1 },
                MapElement::Empty,
            ],
            vec![
                MapElement::Number { count: 1 },
                MapElement::Number { count: 2 },
                MapElement::Mine,
                MapElement::Number { count: 2 },
                MapElement::Number { count: 1 },
            ],
            vec![
                MapElement::Empty,
                MapElement::Number { count: 1 },
                MapElement::Number { count: 2 },
                MapElement::Mine,
                MapElement::Number { count: 1 },
            ],
        ];
        assert_eq!(board_with_numbers.map, expected_map);
    }
}
