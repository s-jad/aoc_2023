fn parse(input: &str) -> [[usize; 4]; 2] {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter(|s| s.chars().all(char::is_numeric))
                .map(|n| n.parse::<usize>().expect("Should be number"))
                .collect::<Vec<usize>>()
                .try_into()
                .expect("Should be of length 4")
        })
        .collect::<Vec<[usize; 4]>>()
        .try_into()
        .expect("Should be 2 lines")
}

// returns the product from the numbers of ways
// that you can beat each race.
// E.g. 4 ways to beat race_1, 3 to beat race_2, 2 to beat race_3 =>
// 4 * 3 * 2 = 24
fn process(input: &str) -> usize {
    let [times, distance] = parse(input);

    times
        .iter()
        .enumerate()
        .map(|(idx, &t)| {
            let mut ways = 0;
            for remaining_time in (0..t).rev() {
                let speed = t - remaining_time;

                let disance_travelled = speed * remaining_time;

                if disance_travelled > distance[idx] {
                    ways += 1;
                }
            }
            ways
        })
        .product()
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}
