use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let mut paras = input
        .split("\n\n")
        .into_iter()
        .map(|p| {
            p.lines()
                .filter_map(|l| {
                    let st = l
                        .split_whitespace()
                        .into_iter()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect_vec();
                    if st.len() == 0 {
                        None
                    } else {
                        Some(st)
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let seeds = paras[0].clone().into_iter().flatten().collect_vec();
    let maps = paras[1..]
        .iter()
        .map(|v| {
            v.iter()
                .map(|m| ((m[0]..(m[0] + m[2])), (m[1]..(m[1] + m[2]))))
                .collect_vec()
        })
        .collect_vec();

    let mut start_vals = seeds;
    for maps_idx in 0..maps.len() {
        let new_vals = start_vals
            .iter()
            .map(|val| {
                let ranges = maps[maps_idx].iter().find(|&m| m.1.contains(&val));

                if ranges.is_some() {
                    let (dest, start) = ranges.unwrap();
                    let diff = val - start.start;
                    let dest_val = dest.start + diff;
                    dest_val
                } else {
                    *val
                }
            })
            .collect_vec();

        start_vals = new_vals;
    }

    *start_vals.iter().min().unwrap()
}

fn main() {
    let input = include_str!("../../input.txt");
    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}
