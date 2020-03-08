use colored::Colorize;
use rand::Rng;
use std::io;

fn main() {
    loop {
        let length = rand::thread_rng().gen_range(10, 40);
        let input: Vec<i32> = (0..length)
            .map(|_| rand::thread_rng().gen_range(0, 7))
            .collect();

        let max: i32 = *input.iter().max().expect("oops, there was no max");

        let mut ocounter = 0;
        let map = (0..max)
            .rev()
            .map(|i: i32| {
                let map = generate_map_line(&input, i);
                let (map, water_count) = waterize_map_line(&map);
                ocounter = ocounter + water_count;
                map
            })
            .collect::<Vec<String>>();

        colorized_print_map(map);
        println!("result: {}", ocounter);

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");
    }
}

fn generate_map_line(input: &Vec<i32>, i: i32) -> String {
    input
        .iter()
        .map(|x: &i32| {
            if *x > i {
                String::from("H")
            } else {
                String::from(" ")
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn waterize_map_line(map: &String) -> (String, i32) {
    let mut ret = String::new();
    let mut open = false;
    let mut counter = 0;
    let mut ocounter = 0;
    for c in map.chars() {
        if !open {
            if c == 'H' {
                open = true;
                ret.push_str("H");
            } else {
                ret.push_str(" ");
            }
        } else {
            if c == 'H' {
                for _ in 0..counter {
                    ret.push_str("O");
                    ocounter = ocounter + 1;
                }
                counter = 0;
                ret.push_str("H");
            } else {
                counter = counter + 1;
            }
        }
    }
    for _ in 0..counter {
        ret.push_str(" ");
    }
    (ret, ocounter)
}

fn colorized_print_map(map: Vec<String>) {
    for i in map {
        for x in i.chars() {
            let c = if x == 'H' {
                " ".on_red()
            } else if x == 'O' {
                " ".on_blue()
            } else {
                " ".normal()
            };
            print!("{}", c);
        }
        println!("");
    }
}
