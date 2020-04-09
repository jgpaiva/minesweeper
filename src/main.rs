use colored::Colorize;
use rand::Rng;
use std::io;

use cargotest::create_board;
use cargotest::numbers_on_board;
use cargotest::open_item;
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
        if matches!(board.state, BoardState::Failed) {
            return;
        }

        let column = read_char("please input column:");
        let column = coord_reverse_mapping(column);
        let row = read_char("please input row:");
        let row = coord_reverse_mapping(row);
        let p = Point { x: column, y: row };

        board = open_item(board, p);
    }
}

fn read_char(message: &str) -> u8 {
    println!("{}", message);
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read line");
    line.as_bytes().first().unwrap().clone()
}

fn coord_reverse_mapping(c: u8) -> i32 {
    let mut mapping = vec![];
    mapping.extend((b'0'..=b'9').map(char::from));
    mapping.extend((b'a'..=b'z').map(char::from));
    let c = char::from(c);

    let v = mapping
        .iter()
        .enumerate()
        .filter(|(i, &x)| c == x)
        .nth(0)
        .unwrap();
    v.0 as i32
}

fn print_board_state(board: &cargotest::Board) {
    print!("Board is currently ");
    match board.state {
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
                Some(MapElement::Mine { open: open }) => {
                    if is_failed || *open {
                        " ".on_red()
                    } else {
                        " ".on_yellow()
                    }
                }
                Some(MapElement::Empty { open: open }) => {
                    if is_failed || *open {
                        " ".on_bright_white()
                    } else {
                        " ".on_yellow()
                    }
                }
                Some(MapElement::Number { open: open, count }) => {
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
