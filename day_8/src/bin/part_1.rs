use itertools::Itertools;

fn process(input: &str) -> u64 {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();

    let rest: Vec<(&str, (&str, &str))> = lines
        .flat_map(|line| {
            line.split_terminator(&[' ', '=', '(', ')', ','][..])
                .filter(|&s| s.chars().all(|c| c.is_alphabetic()))
                .filter(|&s| !s.is_empty())
                .tuples::<(_, _, _)>()
                .map(|(a, b, c)| (a, (b, c)))
                .collect::<Vec<(&str, (&str, &str))>>()
        })
        .collect();

    println!("instructions => {:?}", instructions);
    println!("rest => {:?}", rest);

    let mut i_cycle = instructions.iter().cycle();

    let mut current_map = "AAA";
    let mut count = 0;

    while current_map != "ZZZ" {
        let instruction = i_cycle.next().unwrap();
        let map_pos = rest.iter().position(|&(m, _)| m == current_map).unwrap();

        match instruction {
            'L' => {
                current_map = rest[map_pos].1 .0;
            }
            'R' => {
                current_map = rest[map_pos].1 .1;
            }
            _ => {
                println!("Shouldnt be here!");
            }
        }
        count += 1;
    }

    count
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(2, output);
    }

    #[test]
    fn test_2() {
        let input = include_str!("../../test_2.txt");
        let output = process(input);
        assert_eq!(6, output);
    }
}
