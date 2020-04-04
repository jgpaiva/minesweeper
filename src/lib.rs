use lazy_static::lazy_static;

#[derive(Debug, PartialEq)]
pub enum MapElement {
    Land,
    Water,
    Air,
    Mine,
    Empty,
}

pub fn solve_problem(input: &[i32]) -> (Vec<Vec<MapElement>>, i32) {
    let max: i32 = *input.iter().max().expect("oops, there was no max");

    let mut ocounter = 0;
    let map = (0..max)
        .rev()
        .map(|i: i32| {
            let map = generate_map_line(&input, i);
            let (map, water_count) = waterize_map_line(&map);
            ocounter += water_count;
            map
        })
        .collect::<Vec<Vec<MapElement>>>();
    (map, ocounter)
}

/// Generates a map line from the input array and the current line
fn generate_map_line(input: &[i32], i: i32) -> Vec<MapElement> {
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

fn waterize_map_line(map: &[MapElement]) -> (Vec<MapElement>, i32) {
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
                        ocounter += 1;
                    }
                    counter = 0;
                    ret.push(MapElement::Land);
                }
                MapElement::Air => {
                    counter += 1;
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

fn create_board(_width: i32, _height:i32, _mines: i32, _generator: Box<dyn FnMut(i32, i32) -> i32> ) -> std::vec::Vec<std::vec::Vec<MapElement>>{
    vec![
        vec![
            MapElement::Mine,
            MapElement::Empty,
            MapElement::Empty,
            MapElement::Empty,
        ],
        vec![
            MapElement::Empty,
            MapElement::Mine,
            MapElement::Empty,
            MapElement::Empty,
        ],
        vec![
            MapElement::Empty,
            MapElement::Empty,
            MapElement::Mine,
            MapElement::Empty,
        ],
        vec![
            MapElement::Empty,
            MapElement::Empty,
            MapElement::Empty,
            MapElement::Mine,
        ],
    ]}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_problem() {
        let input = vec![3, 0, 1, 4];
        let (map, water_counter) = solve_problem(&input);
        let expected_map = vec![
            vec![
                MapElement::Air,
                MapElement::Air,
                MapElement::Air,
                MapElement::Land,
            ],
            vec![
                MapElement::Land,
                MapElement::Water,
                MapElement::Water,
                MapElement::Land,
            ],
            vec![
                MapElement::Land,
                MapElement::Water,
                MapElement::Water,
                MapElement::Land,
            ],
            vec![
                MapElement::Land,
                MapElement::Water,
                MapElement::Land,
                MapElement::Land,
            ],
        ];
        assert_eq!(map, expected_map);
        assert_eq!(water_counter, 5);
    }

    //struct RandValues<'a>{
        //vec: &'a std::vec::Vec<i32>,
        //iter: std::slice::Iter<i32><'a>
    //}

    #[test]
    fn test_create_board() {
        let width = 4;
        let height = 4;
        let mines = 4;
        lazy_static! {
            static ref z: &'static Vec<i32> = vec![0,0,1,1,2,2,3,3];
        }
        let iter: &'static std::slice::Iter<i32> = &Box::new(z.iter());
        //#let vec = RandValues { vec: z, iter: z.iter()};
        let rand: Box<dyn FnMut(i32, i32) -> i32> = Box::new(|_start: i32, _end: i32| {
            let v: &i32 = iter.next().unwrap();
            return *v;
        });
        let board = create_board(width, height, mines, rand);
        let expected_board = vec![
            vec![
                MapElement::Mine,
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Empty,
            ],
            vec![
                MapElement::Empty,
                MapElement::Mine,
                MapElement::Empty,
                MapElement::Empty,
            ],
            vec![
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Mine,
                MapElement::Empty,
            ],
            vec![
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Empty,
                MapElement::Mine,
            ],
        ];
        assert_eq!(board, expected_board);
    }
}
