use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

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
                let dist = grid[y][x..grid_row]
                    .iter()
                    .enumerate()
                    .find(|(_, c)| !c.is_digit(10));

                if dist.is_some() {
                    let d = dist.unwrap().0;
                    let num = grid[y][x..(x + d)]
                        .into_iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .expect("should be number");
                    num_coords.push((num, y, (x..(x + d))));
                    x += d as usize - 1;
                } else {
                    let d = grid_row - x;
                    let num = grid[y][x..(x + d)]
                        .into_iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .expect("should be number");
                    num_coords.push((num, y, (x..(x + d))));
                }
            }

            x += 1;
        }
        x = 0;
        y += 1;
    }

    let validate = |x: usize, y: usize| -> usize {
        let symbol_neighbours = [
            (0, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (1, 0),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];

        let connected_nums = (symbol_neighbours)
            .into_iter()
            .filter_map(|(nx, ny)| {
                let dx = nx + x as i32;
                let dy = ny + y as i32;

                if dx < 0 || dx > grid_row as i32 || dy < 0 || dy > grid_col as i32 {
                    None
                } else {
                    Some((dx, dy))
                }
            })
            .filter_map(|(dx, dy)| {
                num_coords
                    .iter()
                    .find(|(_, y, x_range)| y == &(dy as usize) && x_range.contains(&(dx as usize)))
            })
            .collect::<HashSet<_>>();

        if connected_nums.len() == 2 {
            connected_nums.iter().map(|(n, _, _)| n).product()
        } else {
            0
        }
    };

    symbol_coords.into_iter().map(|(x, y)| validate(x, y)).sum()
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
