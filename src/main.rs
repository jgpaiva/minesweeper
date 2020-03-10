use colored::Colorize;
use rand::Rng;
use std::io;

use cargotest::solve_problem;
use cargotest::MapElement;

fn main() {
    loop {
        let length = rand::thread_rng().gen_range(10, 40);
        let input: Vec<i32> = (0..length)
            .map(|_| rand::thread_rng().gen_range(0, 7))
            .collect();

        let (map, water_counter) = solve_problem(&input);

        colorized_print_map(map);
        println!("result: {}", water_counter);

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");
    }
}

fn colorized_print_map(map: Vec<Vec<MapElement>>) {
    for i in map {
        for x in i {
            let c = match x {
                MapElement::Land => " ".on_red(),
                MapElement::Water => " ".on_blue(),
                MapElement::Air => " ".normal(),
            };
            print!("{}", c);
        }
        println!();
    }
}
