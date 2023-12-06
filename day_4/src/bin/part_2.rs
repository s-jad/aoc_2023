type Card = (Vec<u8>, Vec<u8>);
type Game = Vec<Card>;

fn get_initial_game(input: &str) -> Game {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let parts: Vec<&str> = line[9..].split('|').collect();
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

fn get_card_value(card: &Card) -> usize {
    let val = card.1.iter().filter(|&x| card.0.contains(x)).count();

    val
}

fn process(input: &str) -> usize {
    let game: Game = get_initial_game(input);
    let mut copies: Vec<usize> = Vec::new();

    for _ in 0..game.len() {
        copies.push(1);
    }

    game.iter()
        .enumerate()
        .map(|(card_idx, card)| {
            let mut total_cards: usize = 0;
            for _ in 0..copies[card_idx] {
                let val = get_card_value(card);
                for j in (card_idx + 1)..(card_idx + 1 + val) {
                    copies[j] += 1;
                }
                total_cards += 1
            }
            total_cards
        })
        .sum::<usize>()
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
