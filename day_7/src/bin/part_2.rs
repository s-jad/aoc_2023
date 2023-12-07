use itertools::Itertools;
use std::collections::HashMap;

fn get_card_strength_map() -> HashMap<char, u8> {
    let mut m = HashMap::new();
    m.insert('A', 14);
    m.insert('K', 13);
    m.insert('Q', 12);
    m.insert('T', 10);
    m.insert('9', 9);
    m.insert('8', 8);
    m.insert('7', 7);
    m.insert('6', 6);
    m.insert('5', 5);
    m.insert('4', 4);
    m.insert('3', 3);
    m.insert('2', 2);
    m.insert('J', 1);

    m
}

fn get_hand_strength_map() -> HashMap<(u8, u8), u8> {
    let mut m: HashMap<(u8, u8), u8> = HashMap::new();
    m.insert((5, 0), 6);
    m.insert((4, 1), 5);
    m.insert((3, 2), 4);
    m.insert((3, 1), 3);
    m.insert((2, 2), 2);
    m.insert((2, 1), 1);
    m.insert((1, 1), 0);

    m
}

fn get_hand_type(card: &str) -> HashMap<char, u8> {
    let mut counts: HashMap<char, u8> = HashMap::new();
    let mut jokerified_counts: HashMap<char, u8> = HashMap::new();

    for c in card.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    if counts.len() > 1 {
        let max_char = counts
            .iter()
            .filter(|(&c, _)| c != 'J')
            .max_by_key(|&(_, &count)| count)
            .map(|(&char, _)| char.to_string())
            .expect("Should be present as it was in the hashmap");

        let jokerified_card = card.replace('J', &max_char);

        for c in jokerified_card.chars() {
            *jokerified_counts.entry(c).or_insert(0) += 1;
        }

        if jokerified_counts.len() <= 1 {
            jokerified_counts.insert('X', 0);
        }

        jokerified_counts
    } else {
        if counts.len() <= 1 {
            counts.insert('X', 0);
        }
        counts
    }
}

fn cmp_ranks(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    for (idx, k_a) in a.iter().enumerate() {
        if k_a > &b[idx] {
            return true;
        } else if k_a < &b[idx] {
            return false;
        }
    }

    false
}

fn sort_by_card_rank(unsorted: Vec<(u8, Vec<u8>, u16)>) -> Vec<(u8, Vec<u8>, u16)> {
    let mut sorting_vec = unsorted;

    for i1 in 0..sorting_vec.len() {
        let mut i2 = i1 + 1;
        while i2 < sorting_vec.len() {
            if cmp_ranks(&sorting_vec[i1].1, &sorting_vec[i2].1) {
                let tmp = sorting_vec[i1].clone();
                sorting_vec[i1] = sorting_vec[i2].clone();
                sorting_vec[i2] = tmp;
            }
            i2 += 1;
        }
    }

    sorting_vec
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (
                parts[0], // original str
                get_hand_type(parts[0]),
                parts[1].parse::<u16>().expect("Should be a number"),
            )
        })
        .map(|(original_s, cards, bet)| {
            let hand_strength_map = get_hand_strength_map();
            let mut card_amounts: Vec<_> = cards.values().collect();
            card_amounts.sort_unstable();
            let max_amounts = (
                *card_amounts[card_amounts.len() - 1],
                *card_amounts[card_amounts.len() - 2],
            );

            let hand_strength = hand_strength_map
                .get(&max_amounts)
                .expect("Has a hand strength");

            let s_val_arr = original_s
                .chars()
                .map(|c| {
                    let card_strength_map = get_card_strength_map();
                    *card_strength_map.get(&c).expect("Should have a value")
                })
                .collect_vec();

            (*hand_strength, s_val_arr, bet)
        })
        .sorted_unstable_by_key(|k| k.0)
        .group_by(|k| k.0)
        .into_iter()
        .flat_map(|(_, group)| {
            let unsorted_group = group.collect_vec();
            sort_by_card_rank(unsorted_group)
        })
        .enumerate()
        .map(|(pos, (_, _, bet))| (pos + 1) * bet as usize)
        .sum()
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
    fn test_get_hand_type() {
        let input = "KTA33";
        let mut map: HashMap<char, u8> = HashMap::new();
        map.insert('K', 1);
        map.insert('T', 1);
        map.insert('A', 1);
        map.insert('3', 2);
        assert_eq!(map, get_hand_type(input));
    }

    #[test]
    fn test_2() {
        let input = include_str!("../../test_2.txt");
        assert_eq!(6839, process(input));
    }
}
