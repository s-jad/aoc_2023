use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn gcd(u: usize, v: usize) -> usize {
    let mut u = u;
    let mut v = v;

    if u == 0 {
        return v as usize;
    }
    if v == 0 {
        return u as usize;
    }

    let gcd_exponent_of_two = (u | v).trailing_zeros();

    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    while u != v {
        if u < v {
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }
    (u << gcd_exponent_of_two) as usize
}

fn lcm(a: usize, b: usize) -> usize {
    let mut x;
    let mut y;

    if a > b {
        x = a;
        y = b;
    } else {
        x = b;
        y = a;
    }

    let mut rem = x % y;

    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }

    a * b / y
}

fn lcm_multiple(n: &[usize]) -> usize {
    let mut res = n[0];

    for &num in &n[1..] {
        res = lcm(res, num);
    }

    res
}

fn process(input: &str) -> usize {
    let (mut icycle, m) = (
        input.lines().next().unwrap().chars().cycle(),
        input.lines().skip(2),
    );

    let maps = m
        .map(|l| {
            l.split_terminator(&[',', ' ', '\n', '=', '(', ')'])
                .filter(|s| !s.is_empty())
                .map(|s| s)
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .fold(HashMap::new(), |mut acc, (key, l, r)| {
            acc.insert(key, (l, r));
            acc
        });

    let mut paths = maps
        .keys()
        .filter(|k| k.chars().last().unwrap() == 'A')
        .map(|s| (*s, false))
        .collect_vec();

    const NUM_PATHS: usize = 6;
    let mut counts = [0; NUM_PATHS];
    let mut totals = [0; NUM_PATHS];
    let mut pidx = 0;
    let mut remaining_paths = NUM_PATHS;

    while remaining_paths != 0 {
        let next = icycle.next().unwrap();
        for path in paths.iter_mut() {
            if !path.1 {
                let n = match next {
                    'L' => maps.get(path.0).unwrap().0,
                    'R' => maps.get(path.0).unwrap().1,
                    _ => unreachable!(),
                };

                if n.chars().last().unwrap() == 'Z' {
                    counts[pidx] += 1;
                    totals[pidx] = counts[pidx];
                    path.1 = true;
                    remaining_paths -= 1;
                } else {
                    path.0 = n;
                    counts[pidx] += 1;
                }
            }
            pidx = (pidx + 1) % NUM_PATHS;
        }
    }

    lcm_multiple(&counts)
}

fn main() {
    let input = include_str!("../../input.txt");

    let t = Instant::now();
    let output = process(input);
    let time = t.elapsed();

    dbg!(output, time);
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
