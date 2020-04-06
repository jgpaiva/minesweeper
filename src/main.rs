use colored::Colorize;
use rand::Rng;
use std::io;

use cargotest::create_board;
use cargotest::numbers_on_board;
use cargotest::MapElement;

fn main() {
    loop {
        //let height = rand::thread_rng().gen_range(5, 30);
        //let width = rand::thread_rng().gen_range(5, 30);
        //let mines = rand::thread_rng().gen_range((height * width) / 4, (height * width) * 5 / 10);
        let width = 30;
        let height = 16;
        let mines = 99;

        let board = create_board(width, height, mines, |x, y| {
            rand::thread_rng().gen_range(x, y)
        });

        let board = numbers_on_board(board);

        colorized_print_map(board);

        break;

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");
    }
}

fn colorized_print_map(board: cargotest::Board) {
    let mut mapping = vec![];
    mapping.extend((b'0'..=b'9').map(char::from));
    mapping.extend((b'a'..=b'z').map(char::from));
    print!("  ");
    for i in 0..board.width {
        print!("{}", mapping[i]);
    }
    println!();
    for y in 0..board.height {
        print!("{} ", mapping[y]);
        for x in 0..board.width {
            let x = x as i32;
            let y = y as i32;
            let c = match board.at(&cargotest::Point { x, y }) {
                Some(MapElement::Mine) => " ".on_red(),
                Some(MapElement::Empty) => " ".on_bright_white(),
                Some(MapElement::Number { count }) => format!("{}", count).black().on_bright_cyan(),
                _ => unreachable!(),
            };
            print!("{}", c);
        }
        print!(" {}", mapping[y]);
        println!();
    }

    print!("  ");
    for i in 0..board.width {
        print!("{}", mapping[i]);
    }
    println!();
}
