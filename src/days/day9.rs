use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let input: Vec<Vec<u32>> = get_input::<Input>("day-9").0;
    let rows = input.len();
    let cols = input[0].len();
    let mut low_points = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            let height = input[i][j];
            let mut is_low = true;
            let check_up = i > 0;
            let check_down = i < (rows - 1);
            let check_left = j > 0;
            let check_right = j < (cols - 1);
            if check_up && height >= input[i - 1][j] {
                is_low = false;
            }
            if check_down && height >= input[i + 1][j] {
                is_low = false;
            }
            if check_left && height >= input[i][j - 1] {
                is_low = false;
            }
            if check_right && height >= input[i][j + 1] {
                is_low = false;
            }
            if is_low {
                low_points.push((i, j));
            }
        }
    }
    let mut basins = Vec::new();
    for point in low_points {
        let mut basin = HashSet::new();
        basin.insert(point);
        let mut basin_size = basin.len();
        let mut another_pass = true;
        while another_pass {
            let mut new_points = HashSet::new();
            for point in &basin {
                let adj_points = |(x, y)| {
                    let mut points = Vec::new();
                    if x > 0 {
                        points.push((x - 1, y));
                    }
                    if x < (rows - 1) {
                        points.push((x + 1, y));
                    }
                    if y > 0 {
                        points.push((x, y - 1));
                    }
                    if y < (cols - 1) {
                        points.push((x, y + 1));
                    }
                    points
                };
                for adj_point in adj_points(*point) {
                    if adj_points(adj_point).iter().all(|(x, y)| {
                        (basin.contains(&(*x, *y))
                            || input[*x as usize][*y as usize] >= input[adj_point.0][adj_point.1])
                            && input[adj_point.0][adj_point.1] != 9
                    }) {
                        new_points.insert(adj_point);
                    }
                }
            }
            basin.extend(&new_points);
            if basin_size == basin.len() {
                another_pass = false;
            }
            basin_size = basin.len();
        }
        let new_basin = basin
            .iter()
            .map(|(x, y)| input[*x as usize][*y as usize])
            .collect::<Vec<_>>();
        basins.push(new_basin);
    }
    let basin_sizes = basins
        .iter()
        .map(|basin: &Vec<u32>| basin.clone().len())
        .sorted()
        .rev()
        .collect::<Vec<_>>();
    let product = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];
    dbg!(product);
}

fn part1() {
    let input: Vec<Vec<u32>> = get_input::<Input>("day-9").0;
    let rows = input.len();
    let cols = input[0].len();
    let mut low_points = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            let height = input[i][j];
            let mut is_low = true;
            let check_up = i > 0;
            let check_down = i < (rows - 1);
            let check_left = j > 0;
            let check_right = j < (cols - 1);
            if check_up && height >= input[i - 1][j] {
                is_low = false;
            }
            if check_down && height >= input[i + 1][j] {
                is_low = false;
            }
            if check_left && height >= input[i][j - 1] {
                is_low = false;
            }
            if check_right && height >= input[i][j + 1] {
                is_low = false;
            }
            if is_low {
                low_points.push(1 + height);
            }
        }
    }
    let risk = low_points.iter().sum::<u32>();
    dbg!(risk);
}

#[derive(Clone, Deserialize)]
struct Input(Vec<Vec<u32>>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
