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
    let mut row_vec = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut y_idx = 0;

    while y_idx < row_vec.len() {
        if row_vec[y_idx].iter().all(|c| c == &'.') {
            row_vec.insert(y_idx + 1, row_vec[y_idx].clone());
            y_idx += 2;
        } else {
            y_idx += 1;
        }
    }

    let mut col_vec = transpose2(row_vec);

    let mut x_idx = 0;

    while x_idx < col_vec.len() {
        if col_vec[x_idx].iter().all(|c| c == &'.') {
            col_vec.insert(x_idx + 1, col_vec[x_idx].clone());
            x_idx += 2;
        } else {
            x_idx += 1;
        }
    }

    let galaxies = col_vec
        .into_iter()
        .enumerate()
        .flat_map(|(x, inner)| {
            inner
                .into_iter()
                .enumerate()
                .map(move |(y, c)| ((x, y), c))
                .filter(|((_, _), c)| c == &'#')
                .map(|((x, y), _)| (x, y))
                .collect_vec()
        })
        .collect_vec();

    let grid = Grid::from_coordinates(&galaxies).unwrap();

    let mut distances: Vec<usize> = Vec::new();

    for i in 0..(galaxies.len() - 1) {
        for j in (i + 1)..galaxies.len() {
            distances.push(grid.distance(galaxies[i], galaxies[j]));
        }
    }

    println!("grid =>\n{:?}\n", grid);
    distances.iter().sum::<usize>()
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}
