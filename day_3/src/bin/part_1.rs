use itertools::Itertools;
use std::{ops::Range, time::Instant};

fn process(input: &str) -> usize {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let mut num_coords = vec![];
    let mut symbol_coords = vec![];
    let (grid_row, grid_col) = (grid[0].len(), grid.len());

    let mut x = 0;
    let mut y = 0;

    while y < grid_col {
        while x < grid_row {
            if grid[y][x].is_ascii_punctuation() && grid[y][x] != '.' {
                symbol_coords.push((x, y));
            }

            if grid[y][x].is_digit(10) {
                let start_x = x as i32 - 1;

                let dist = grid[y][x..grid_row]
                    .iter()
                    .enumerate()
                    .find(|(_, c)| !c.is_digit(10));

                if dist.is_some() {
                    let d = dist.unwrap().0;

                    num_coords.push((y as i32, (start_x..(x + d) as i32)));
                    x += d as usize - 1;
                } else {
                    let d = grid_row - x;
                    num_coords.push((y as i32, (start_x..(x + d) as i32)));
                }
            }

            x += 1;
        }
        x = 0;
        y += 1;
    }
    let validate = |y: i32, x_range: Range<i32>| -> Option<usize> {
        let y_neighbours = [y - 1, y, y + 1];

        let st: usize = x_range.start.try_into().unwrap_or(0usize);
        let end = x_range.end as usize;

        if ((x_range.start)..=end as i32)
            .into_iter()
            .cartesian_product(y_neighbours.into_iter())
            .filter(|(_, y)| y >= &0 && y < &(grid_col as i32))
            .any(|(x, y)| symbol_coords.contains(&(x as usize, y as usize)))
        {
            let num = grid[y as usize][st..end]
                .iter()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<usize>()
                .expect("expected parse to num");
            return Some(num);
        }
        return None;
    };

    num_coords
        .into_iter()
        .filter_map(|(y, x_range)| validate(y, x_range))
        .sum()
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}
