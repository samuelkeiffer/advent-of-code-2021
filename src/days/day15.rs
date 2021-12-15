use crate::*;
use pathfinding::prelude::astar;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let Input(grid) = get_input::<Input>("day-15");
    let mut big_grid = Vec::<Vec<usize>>::new();
    for i in 0..5 {
        let mut big_row = vec![Vec::<usize>::new(); grid.len()];
        for j in 0..5 {
            let mut grid_portion = grid
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|risk| {
                            let modulus = (risk + i + j) % 9;
                            if modulus == 0 {
                                9
                            } else {
                                modulus
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            big_row
                .iter_mut()
                .zip(grid_portion.iter_mut())
                .for_each(|(row, extend)| row.append(extend))
        }
        big_grid.append(&mut big_row);
    }
    let cost = find_path_cost(&big_grid);
    dbg!(cost);
}

fn part1() {
    let Input(grid) = get_input::<Input>("day-15");
    let cost = find_path_cost(&grid);
    dbg!(cost);
}

fn find_path_cost(grid: &[Vec<usize>]) -> usize {
    let start_point: (usize, usize) = (0, 0);
    let end_point = (grid.len() - 1, grid[0].len() - 1);
    let (_path, cost) = astar(
        &start_point,
        |(x, y)| {
            [
                (x + 1, *y),
                (*x, y + 1),
                ((*x).max(1) - 1, *y),
                (*x, (*y).max(1) - 1),
            ]
            .iter()
            .filter_map(|(x, y)| {
                grid.get(*x)
                    .and_then(|row| row.get(*y))
                    .map(|cost| ((*x, *y), *cost))
            })
            .collect::<Vec<_>>()
        },
        |(x, y)| end_point.0 - x + end_point.1 - y,
        |point| *point == end_point,
    )
    .unwrap();
    cost
}

#[derive(Clone, Deserialize)]
struct Input(Vec<Vec<usize>>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
