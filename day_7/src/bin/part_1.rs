use itertools::Itertools;
use std::collections::HashMap;

fn get_card_strength_map() -> HashMap<char, u8> {
    let mut m = HashMap::new();
    m.insert('A', 14);
    m.insert('K', 13);
    m.insert('Q', 12);
    m.insert('J', 11);
    m.insert('T', 10);
    m.insert('9', 9);
    m.insert('8', 8);
    m.insert('7', 7);
    m.insert('6', 6);
    m.insert('5', 5);
    m.insert('4', 4);
    m.insert('3', 3);
    m.insert('2', 2);

    m
}

fn get_hand_strength_map() -> HashMap<(u8, u8), u8> {
    let mut map: HashMap<(u8, u8), u8> = HashMap::new();
    map.insert((5, 0), 6);
    map.insert((4, 1), 5);
    map.insert((3, 2), 4);
    map.insert((3, 1), 3);
    map.insert((2, 2), 2);
    map.insert((2, 1), 1);
    map.insert((1, 1), 0);

    map
}

fn get_hand_type(card: &str) -> HashMap<u8, u8> {
    let mut counts: HashMap<u8, u8> = HashMap::new();
    let card_strength_map: HashMap<char, u8> = get_card_strength_map();

    for c in card.chars() {
        let num = card_strength_map
            .get(&c)
            .expect("Key should be present in card strength map");
        *counts.entry(*num).or_insert(0) += 1;
    }

    if counts.len() <= 1 {
        counts.insert(0, 0);
    }

    counts
}

fn cmp_ranks(a: &HashMap<u8, u8>, b: &HashMap<u8, u8>) -> bool {
    let mut a_vals = a
        .iter()
        .map(|(&k, &v)| k as u32 + v as u32 * 100)
        .collect_vec();
    let mut b_vals = b
        .iter()
        .map(|(&k, &v)| k as u32 + v as u32 * 100)
        .collect_vec();

    a_vals.sort_by_key(|&k| k);
    b_vals.sort_by_key(|&k| k);

    for (idx, k_a) in a_vals.iter().enumerate().rev() {
        if k_a > &b_vals[idx] {
            return true;
        } else if k_a < &b_vals[idx] {
            return false;
        }
    }

    false
}

fn sort_by_card_rank(unsorted: Vec<(u8, HashMap<u8, u8>, u16)>) -> Vec<(u8, HashMap<u8, u8>, u16)> {
    let mut sorting_vec = unsorted;

    for i1 in 0..sorting_vec.len() {
        let mut i2 = i1 + 1;
        while i2 < sorting_vec.len() {
            if cmp_ranks(&sorting_vec[i1].1, &sorting_vec[i2].1) {
                println!("cmp_ranks true! swapping");
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
    let hands = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (
                get_hand_type(parts[0]),
                parts[1].parse::<u16>().expect("Should be a number"),
            )
        })
        .map(|(cards, bet)| {
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
            (*hand_strength, cards, bet)
        })
        .sorted_unstable_by_key(|k| k.0)
        .collect_vec();

    println!("hands => {:?}", hands);

    let sorted_hands = hands
        .into_iter()
        .group_by(|k| k.0)
        .into_iter()
        .flat_map(|(_, group)| {
            let unsorted_group = group.collect_vec();
            sort_by_card_rank(unsorted_group)
        })
        .enumerate()
        .map(|(pos, (rank, card, bet))| (pos + 1, (rank, card, bet)))
        .map(|(pos, (_, _, bet))| pos * bet as usize)
        .collect_vec();

    println!("sorted_hands => {:?}", sorted_hands);
    sorted_hands.iter().sum()
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
        let mut map: HashMap<u8, u8> = HashMap::new();
        map.insert(13, 1);
        map.insert(10, 1);
        map.insert(14, 1);
        map.insert(3, 2);
        assert_eq!(map, get_hand_type(input));
    }

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        assert_eq!(6440, process(input));
    }
}
