use colored::Colorize;
use rand::Rng;
use std::io;

enum MapElement {
    Land,
    Water,
    Air,
}

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
            .collect::<Vec<Vec<MapElement>>>();

        colorized_print_map(map);
        println!("result: {}", ocounter);

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");
    }
}

fn generate_map_line(input: &Vec<i32>, i: i32) -> Vec<MapElement> {
    input
        .iter()
        .map(|x: &i32| {
            if *x > i {
                MapElement::Land
            } else {
                MapElement::Air
            }
        })
        .collect::<Vec<MapElement>>()
}

fn waterize_map_line(map: &Vec<MapElement>) -> (Vec<MapElement>, i32) {
    let mut ret: Vec<MapElement> = Vec::new();
    let mut open = false;
    let mut counter = 0;
    let mut ocounter = 0;
    for c in map {
        if !open {
            match c {
                MapElement::Land => {
                    open = true;
                    ret.push(MapElement::Land);
                }
                MapElement::Air => {
                    ret.push(MapElement::Air);
                }
                _ => panic!("at this point a map should not have water"),
            }
        } else {
            match c {
                MapElement::Land => {
                    for _ in 0..counter {
                        ret.push(MapElement::Water);
                        ocounter = ocounter + 1;
                    }
                    counter = 0;
                    ret.push(MapElement::Land);
                }
                MapElement::Air => {
                    counter = counter + 1;
                }
                _ => panic!("at this point a map should not have water"),
            }
        }
    }
    for _ in 0..counter {
        ret.push(MapElement::Air);
    }
    (ret, ocounter)
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
