fn solve(time: u64, distance: u64) -> u64 {
    let t = time as f64;
    let d = distance as f64;

    let eq = ((t * t - (4.0 * (-d) * -1.0)).sqrt()) / -2.0;

    let lower_bound = (-t / -2.0) + eq;
    let upper_bound = (-t / -2.0) - eq;

    if lower_bound.fract() == 0.0 || upper_bound.fract() == 0.0 {
        (upper_bound - lower_bound) as u64 - 1
    } else {
        upper_bound.floor() as u64 - lower_bound.ceil() as u64 + 1
    }
}

// returns the product from the numbers of ways
// that you can beat each race.
// E.g. 4 ways to beat race_1, 3 to beat race_2, 2 to beat race_3 =>
// 4 * 3 * 2 = 24
fn process(input: &str) -> u64 {
    let mut lines = input.lines();

    lines
        .next()
        .unwrap_or("")
        .split(|c: char| !c.is_digit(10))
        .filter_map(|n| n.parse().ok())
        .zip(
            lines
                .next()
                .unwrap_or("")
                .split(|c: char| !c.is_digit(10))
                .filter_map(|n| n.parse().ok()),
        )
        .map(|(time, distance)| solve(time, distance))
        .product()
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}
