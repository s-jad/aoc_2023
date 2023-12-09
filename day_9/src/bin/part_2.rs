use itertools::Itertools;

fn process(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<isize>().expect("Is a number"))
                .collect_vec()
        })
        .map(|nums| {
            let mut totals_arr: Vec<Vec<isize>> = Vec::new();
            let mut current_nums = nums;
            totals_arr.push(current_nums.clone());
            while !current_nums.windows(2).all(|w| w[0] == w[1]) {
                current_nums = current_nums
                    .into_iter()
                    .tuple_windows::<(_, _)>()
                    .map(|tup| tup.1 - tup.0)
                    .collect_vec();
                totals_arr.push(current_nums.clone());
            }
            totals_arr
        })
        .map(|totals| {
            let mut to_subtract = 0;

            totals
                .into_iter()
                .rev()
                .map(move |mut total| {
                    let first = *total.first().unwrap();
                    let next = first - to_subtract;
                    to_subtract = next;
                    total.push(next);
                    *total.last().unwrap()
                })
                .collect_vec()
        })
        .map(|t| *t.last().unwrap())
        .sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}
