#[derive(Debug, PartialEq)]
pub enum MapElement {
    Land,
    Water,
    Air,
}

pub fn solve_problem(input: &Vec<i32>) -> (Vec<Vec<MapElement>>, i32) {
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
    (map, ocounter)
}

/// Generates a map line from the input array and the current line
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
}
