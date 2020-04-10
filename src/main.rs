use colored::Colorize;
use rand::Rng;
use std::io;

use cargotest::create_board;
use cargotest::numbers_on_board;
use cargotest::BoardState;
use cargotest::MapElement::Mine;
use cargotest::MapElement::Number;
use cargotest::MapElementCellState::Closed;
use cargotest::MapElementCellState::Flagged;
use cargotest::MapElementCellState::Open;
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

        println!("Please input operation (open or flag), column and row.Examples:\no35 to open column 3, row 5\nf13 to flag column 1, row 3");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");
        let op = process_line(line, &board);
        match op {
            Some(Operation::Open { point }) => {
                board = board.cascade_open_item(&point).unwrap_or(board)
            }
            Some(Operation::Flag { point }) => board = board.flag_item(&point),
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
        [op, x, y, b'\n'] => {
            let x = coord_reverse_mapping(*x);
            let y = coord_reverse_mapping(*y);
            let p = Point { x, y };
            if matches!(board.at(&p), Some(_)) {
                match op {
                    b'o' => Some(Operation::Open { point: p }),
                    b'f' => Some(Operation::Flag { point: p }),
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn coord_reverse_mapping(c: u8) -> i32 {
    let mut mapping = vec![];
    mapping.extend(b'0'..=b'9');
    mapping.extend(b'a'..=b'z');

    mapping
        .iter()
        .enumerate()
        .map(|(i, c)| (i as i32, c))
        .find(|(_, &x)| c == x)
        .map(|(i, _)| i)
        .unwrap_or(-1)
}

fn print_board_state(board: &cargotest::Board) {
    print!("Board is currently ");
    match board.state {
        BoardState::Won => print!("{}", "🎉🎉  WON! 🎉🎉".green()),
        BoardState::Playing => print!("{}", "in play".green()),
        BoardState::Failed => print!("{}", "☠️  FAILED ☠️".red()),
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
    let is_done = matches!(board.state, BoardState::Failed | BoardState::Won);
    for y in 0..board.height {
        print!("{} ", mapping[y]);
        for x in 0..board.width {
            let x = x as i32;
            let y = y as i32;
            let c = match board.at(&cargotest::Point { x, y }) {
                Some(Mine { state }) => match (state, is_done) {
                    (_, true) | (Open, _) => " ".on_red(),
                    (Flagged, _) => " ".on_bright_green(),
                    (Closed, _) => " ".on_yellow(),
                },
                Some(Number { state, count: 0 }) => match (state, is_done) {
                    (_, true) | (Open, _) => " ".on_bright_white(),
                    (Flagged, _) => " ".on_bright_green(),
                    (Closed, _) => " ".on_yellow(),
                },
                Some(Number { state, count }) => match (state, is_done) {
                    (_, true) | (Open, _) => format!("{}", count).black().on_bright_cyan(),
                    (Flagged, _) => " ".on_bright_green(),
                    (Closed, _) => " ".on_yellow(),
                },
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
    use pretty_assertions::assert_eq;

    // TODO: I'm a dummy and couldn't figure out how to import this function and the next one from lib.rs
    fn make_map(map: Vec<Vec<(bool, i32)>>) -> Vec<Vec<MapElement>> {
        map.iter()
            .map(|row| {
                row.iter()
                    .map(|(open, count)| match count {
                        -1 => Mine {
                            state: if *open { Open } else { Closed },
                        },
                        count => Number {
                            state: if *open { Open } else { Closed },
                            count: *count,
                        },
                    })
                    .collect()
            })
            .collect()
    }

    pub fn five_by_two_board() -> Board {
        Board::new(make_map(vec![
            vec![(false, -1), (false, 0), (false, 0), (false, 0), (false, 0)],
            vec![(false, 0), (false, -1), (false, 0), (false, 0), (false, 0)],
        ]))
    }

    #[test]
    fn test_process_line() {
        let o = process_line(String::from("o01\n"), &tests::five_by_two_board());
        assert_eq!(
            o,
            Some(Operation::Open {
                point: Point { x: 0, y: 1 }
            })
        );
    }

    #[test]
    fn test_process_line_out_of_bounds_argument() {
        let o = process_line(String::from("o34\n"), &tests::five_by_two_board());
        assert_eq!(o, None);
    }

    #[test]
    fn test_process_line_bad_arguments() {
        let o = process_line(String::from("o\n"), &tests::five_by_two_board());
        assert_eq!(o, None);
    }
}
