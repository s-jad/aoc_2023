fn get_scratch_cards(input: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            println!("line {:?} length => {:?}", i, line.len());
            let parts: Vec<&str> = line[9..].split('|').collect();
            println!("parts {:?} => {:?}", i, parts);
            let card_nums: Vec<u8> = parts[0]
                .split_whitespace()
                .map(|n| n.parse::<u8>().unwrap())
                .collect();
            let my_nums: Vec<u8> = parts[1]
                .split_whitespace()
                .map(|n| n.parse::<u8>().unwrap())
                .collect();
            (card_nums, my_nums)
        })
        .collect()
}

fn get_card_value(card: &(Vec<u8>, Vec<u8>)) -> usize {
    card.1
        .iter()
        .filter(|&x| card.0.contains(x))
        .enumerate()
        .fold(0, |acc, (i, _)| match i {
            0 => acc + 1,
            _ => acc * 2,
        })
}

fn process(input: &str) -> usize {
    let scratch_cards: Vec<(Vec<u8>, Vec<u8>)> = get_scratch_cards(input);
    scratch_cards.iter().map(|card| get_card_value(card)).sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_1() {
    //     let f = include_str!("../../test.txt");
    //     assert_eq!(get_map(f), 4361);
    // }
}
