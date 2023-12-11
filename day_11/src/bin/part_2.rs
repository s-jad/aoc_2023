use itertools::Itertools;
use pathfinding::prelude::Grid;

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn process(input: &str) -> usize {
    let coord_vec = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x, y), c))
                .collect_vec()
        })
        .collect_vec();

    let mut x_millions: Vec<usize> = Vec::new();
    let mut y_millions: Vec<usize> = Vec::new();

    for i in 0..coord_vec.len() {
        if coord_vec[i].iter().all(|((_, _), c)| c == &'.') {
            x_millions.push(i);
        }
    }

    let col_vec = transpose2(coord_vec);

    for i in 0..col_vec.len() {
        if col_vec[i].iter().all(|((_, _), c)| c == &'.') {
            y_millions.push(i);
        }
    }

    let galaxies = col_vec
        .into_iter()
        .flat_map(|inner| {
            inner
                .into_iter()
                .filter(|((_, _), c)| c == &'#')
                .map(|((y, x), _)| (x, y))
                .collect_vec()
        })
        .collect_vec();

    let grid = Grid::from_coordinates(&galaxies).unwrap();

    let mut distances: Vec<usize> = Vec::new();

    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let grid_d = grid.distance(galaxies[i], galaxies[j]);
            let x_factor = x_millions
                .iter()
                .filter(|&x| {
                    x > &galaxies[i].0 && x < &galaxies[j].0
                        || x < &galaxies[i].0 && x > &galaxies[j].0
                })
                .count()
                * 999_999;

            let y_factor = y_millions
                .iter()
                .filter(|&y| {
                    y > &galaxies[i].1 && y < &galaxies[j].1
                        || y < &galaxies[i].1 && y > &galaxies[j].1
                })
                .count()
                * 999_999;

            distances.push(grid_d + x_factor + y_factor);
        }
    }

    distances.iter().sum::<usize>()
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}
