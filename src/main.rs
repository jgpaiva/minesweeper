fn main() {
    let input = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let max: i32 = *input.iter().max().unwrap();

    let res = (0..max + 1)
        .rev()
        .map(|i: i32| {
            let val = input
                .iter()
                .map(|x: &i32| {
                    if x >= &i {
                        String::from("H")
                    } else {
                        String::from(" ")
                    }
                })
                .collect::<Vec<String>>()
                .join("");
            let mut ret = val.clone();
            let mut tentative = String::from("");
            let mut open = false;
            for c in val.chars() {
                if !open {
                    if c == 'H' {
                        open = true;
                        tentative.push_str("H");
                    } else {
                        tentative.push_str(" ");
                    }
                } else {
                    if c == 'H' {
                        tentative.push_str("H");
                        ret = tentative.clone();
                    } else {
                        tentative.push_str("O");
                    }
                }
            }
            ret
        })
        .collect::<Vec<String>>();

    for i in res.clone() {
        println!("{}", i);
    }
    let r: usize = res
        .into_iter()
        .map(|line: String| {
            let a: usize = line
                .chars()
                .map(|c: char| if c == 'O' { 1 } else { 0 })
                .sum();
            a
        })
        .sum();
    println!("result: {}", r)

}
