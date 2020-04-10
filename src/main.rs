use colored::Colorize;
use rand::Rng;
use std::io;

use cargotest::create_board;
use cargotest::numbers_on_board;
use cargotest::BoardState;
use cargotest::MapElement;
use cargotest::Point;

fn main() {
    //let height = rand::thread_rng().gen_range(5, 30);
    //let width = rand::thread_rng().gen_range(5, 30);
    //let mines = rand::thread_rng().gen_range((height * width) / 4, (height * width) * 5 / 10);
    let width = 8;
    let height = 8;
    let mines = 10;

    let board = create_board(width, height, mines, |x, y| {
        rand::thread_rng().gen_range(x, y)
    });

    let mut board = numbers_on_board(board);

    loop {
        colorized_print_map(&board);
        if matches!(board.state, BoardState::Failed | BoardState::Won) {
            return;
        }

        println!("Please input operation, column and row in the following format: ocr.\nExample: o35 to open column 3, row 5");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");
        let op = process_line(line, &board);
        match op {
            Some(Operation::Open { point }) => {
                board = board.cascade_open_item(&point).unwrap_or(board)
            }
            _ => continue,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Open { point: cargotest::Point },
    Flag { point: cargotest::Point },
}

fn process_line(line: String, board: &cargotest::Board) -> Option<Operation> {
    let bytes = line.as_bytes();
    match bytes {
        [b'o', x, y, b'\n'] =>  {
            let x = coord_reverse_mapping(*x);
            let y = coord_reverse_mapping(*y);
            let p = Point { x, y };
            if matches!(board.at(&p), Some(_)) {
                Some(Operation::Open { point: p })
            } else {
                None
            }
        }
        _ => None
    }
}

fn coord_reverse_mapping(c: u8) -> i32 {
    let mut mapping = vec![];
    mapping.extend((b'0'..=b'9').map(char::from));
    mapping.extend((b'a'..=b'z').map(char::from));
    let c = char::from(c);

    let v = mapping
        .iter()
        .enumerate()
        .filter(|(_, &x)| c == x)
        .next()
        .unwrap();
    v.0 as i32
}

fn print_board_state(board: &cargotest::Board) {
    print!("Board is currently ");
    match board.state {
        BoardState::Won => print!("{}", "WON!".green()),
        BoardState::Playing => print!("{}", "in play".green()),
        BoardState::Failed => print!("{}", "failed".red()),
        _ => unreachable!(),
    }
    println!();
}

fn colorized_print_map(board: &cargotest::Board) {
    print_board_state(&board);
    let mut mapping = vec![];
    mapping.extend((b'0'..=b'9').map(char::from));
    mapping.extend((b'a'..=b'z').map(char::from));
    print!("  ");
    for item in mapping.iter().take(board.width) {
        print!("{} ", item);
    }
    println!();
    let is_failed = matches!(board.state, BoardState::Failed);
    for y in 0..board.height {
        print!("{} ", mapping[y]);
        for x in 0..board.width {
            let x = x as i32;
            let y = y as i32;
            let c = match board.at(&cargotest::Point { x, y }) {
                Some(MapElement::Mine { open }) => {
                    if is_failed || *open {
                        " ".on_red()
                    } else {
                        " ".on_yellow()
                    }
                }
                Some(MapElement::Empty { open }) => {
                    if is_failed || *open {
                        " ".on_bright_white()
                    } else {
                        " ".on_yellow()
                    }
                }
                Some(MapElement::Number { open, count }) => {
                    if is_failed || *open {
                        format!("{}", count).black().on_bright_cyan()
                    } else {
                        " ".on_yellow()
                    }
                }
                _ => unreachable!(),
            };
            print!("{} ", c);
        }
        print!("{}", mapping[y]);
        println!();
    }

    print!("  ");
    for item in mapping.iter().take(board.width) {
        print!("{} ", item);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use cargotest::*;

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
    fn test_process_line() {
        let o = process_line(String::from("o01\n"), &two_by_five_board());
        assert_eq!(
            o,
            Some(Operation::Open {
                point: Point { x: 0, y: 1 }
            })
        );
    }

    #[test]
    fn test_process_line_out_of_bounds_argument() {
        let o = process_line(String::from("o34\n"), &two_by_five_board());
        assert_eq!(o, None);
    }

    #[test]
    fn test_process_line_bad_arguments() {
        let o = process_line(String::from("o\n"), &two_by_five_board());
        assert_eq!(o, None);
    }
}
