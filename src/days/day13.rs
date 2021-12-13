use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let Input(dots, folds) = get_input::<Input>("day-13");
    let (max_x, max_y) = dots
        .iter()
        .copied()
        .reduce(|(ax, ay), (bx, by)| (ax.max(bx), ay.max(by)))
        .unwrap();
    let mut field = vec![vec![false; max_y + 1]; max_x + 1];
    for dot in dots {
        field[dot.0][dot.1] = true;
    }
    for fold_ in folds {
        fold(&mut field, fold_.0, fold_.1);
    }
    let display: Vec<_> = field
        .iter()
        .map(|row| {
            row.iter()
                .map(|item| if *item { "#" } else { "." })
                .collect::<Vec<_>>()
        })
        .collect();
    for j in 0..display[0].len() {
        for i in 0..display.len() {
            print!("{}", display[i][j]);
        }
        println!();
    }
}

fn part1() {
    let Input(dots, folds) = get_input::<Input>("day-13");
    let (max_x, max_y) = dots
        .iter()
        .copied()
        .reduce(|(ax, ay), (bx, by)| (ax.max(bx), ay.max(by)))
        .unwrap();
    let mut field = vec![vec![false; max_y + 1]; max_x + 1];
    for dot in dots {
        field[dot.0][dot.1] = true;
    }
    fold(&mut field, folds[0].0, folds[0].1);
    let num_dots = field.iter().flatten().filter(|a| **a).count();
    dbg!(num_dots);
}

fn fold(field: &mut Vec<Vec<bool>>, dir: Dir, line: usize) {
    let length = field.len();
    let width = field[0].len();
    match dir {
        Dir::X => {
            let mut new_field = vec![vec![false; field[0].len()]; line];
            for i in 0..length {
                for j in 0..width {
                    let new_row = match i {
                        i if i > line => line - (i - line),
                        i if i < line => i,
                        _ => continue,
                    };
                    new_field[new_row][j] |= field[i][j];
                }
            }
            *field = new_field;
        }
        Dir::Y => {
            let mut new_field = vec![vec![false; line]; field.len()];
            for i in 0..length {
                for j in 0..width {
                    let new_col = match j {
                        j if j > line => line - (j - line),
                        j if j < line => j,
                        _ => continue,
                    };
                    new_field[i][new_col] |= field[i][j];
                }
            }
            *field = new_field;
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
enum Dir {
    X,
    Y,
}

#[derive(Clone, Deserialize)]
struct Input(Vec<(usize, usize)>, Vec<(Dir, usize)>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
