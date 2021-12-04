#![allow(dead_code)]
use assets_manager::{loader::RonLoader, Asset, AssetCache};
use serde::Deserialize;

fn main() {
    day4_part2();
}

fn day4_part2() {
    let numbers: Vec<u32> = get_input::<Day4Input>("day-4").0;
    let boards_in: Vec<u32> = get_input::<Day4Input>("day-4").1;
    let mut boards: Vec<([[(u32, bool); 5]; 5], bool)> = Vec::new();
    for i in 0..(boards_in.len() / 25) {
        let new_line = [(0, false); 5];
        let mut new_array = [new_line; 5];
        for j in 0..5 {
            for k in 0..5 {
                new_array[j][k] = (boards_in[i * 25 + j * 5 + k], false);
            }
        }
        boards.push((new_array, false));
    }
    let mut finish = None;
    for number in numbers {
        // Set number to true
        for (board, _) in boards.iter_mut() {
            for row in board {
                for (entry, ref mut hit) in row {
                    if *entry == number {
                        *hit = true;
                    }
                }
            }
        }
        // Check if any row or column
        let mut boards_done = Vec::new();
        for (k, (board, _)) in boards.iter().enumerate() {
            for i in 0..5 {
                // Row
                if board[i].iter().all(|(_, hit)| *hit) {
                    boards_done.push(k);
                }
                // Column
                if (0..5).into_iter().all(|j| board[j][i].1) {
                    boards_done.push(k);
                }
            }
        }
        for board in boards_done {
            boards[board].1 = true;
        }
        if boards
            .iter()
            .map(|(_, done)| if *done { 0 } else { 1 })
            .sum::<u32>()
            == 1
        {
            let end = boards.iter().enumerate().find_map(
                |(i, (_, done))| {
                    if *done {
                        None
                    } else {
                        Some(i)
                    }
                },
            );
            finish = end;
        }
        if let Some(finish) = finish {
            if boards[finish].1 {
                let mut sum = 0;
                for i in 0..5 {
                    for j in 0..5 {
                        let (num, hit) = boards[finish].0[i][j];
                        if !hit {
                            sum += num;
                        }
                    }
                }
                let prod = sum * number;
                dbg!(prod);
            }
        }
    }
}

fn day4_part1() {
    let numbers: Vec<u32> = get_input::<Day4Input>("day-4").0;
    let boards_in: Vec<u32> = get_input::<Day4Input>("day-4").1;
    let mut boards: Vec<[[(u32, bool); 5]; 5]> = Vec::new();
    for i in 0..(boards_in.len() / 25) {
        let new_line = [(0, false); 5];
        let mut new_array = [new_line; 5];
        for j in 0..5 {
            for k in 0..5 {
                new_array[j][k] = (boards_in[i * 25 + j * 5 + k], false);
            }
        }
        boards.push(new_array);
    }
    for number in numbers {
        // Set number to true
        for board in boards.iter_mut() {
            for row in board {
                for (entry, ref mut hit) in row {
                    if *entry == number {
                        *hit = true;
                    }
                }
            }
        }
        // Check if any row or column
        let mut done = None;
        for (k, board) in boards.iter().enumerate() {
            for i in 0..5 {
                // Row
                if board[i].iter().all(|(_, hit)| *hit) {
                    done = Some(k);
                }
                // Column
                if (0..5).into_iter().all(|j| board[j][i].1) {
                    done = Some(k)
                }
            }
        }
        if let Some(done) = done {
            let mut sum = 0;
            for i in 0..5 {
                for j in 0..5 {
                    let (num, hit) = boards[done][i][j];
                    if !hit {
                        sum += num;
                    }
                }
            }
            let prod = sum * number;
            dbg!(prod);
            break;
        }
    }
}

#[derive(Clone, Deserialize)]
struct Day4Input(Vec<u32>, Vec<u32>);

impl Asset for Day4Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}

fn day2_part2() {
    let input: Vec<(String, u32)> = get_input::<Day2Input>("day-2").0;
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (dir, dist) in input {
        match dir.as_str() {
            "forward" => {
                pos += dist;
                depth += aim * dist;
            }
            "up" => aim -= dist,
            "down" => aim += dist,
            _ => {}
        }
    }
    let product = pos * depth;
    dbg!(product);
}

fn day2_part1() {
    let input: Vec<(String, u32)> = get_input::<Day2Input>("day-2").0;
    let mut pos = 0;
    let mut depth = 0;
    for (dir, dist) in input {
        match dir.as_str() {
            "forward" => pos += dist,
            "up" => depth -= dist,
            "down" => depth += dist,
            _ => {}
        }
    }
    let product = pos * depth;
    dbg!(product);
}

#[derive(Clone, Deserialize)]
struct Day2Input(Vec<(String, u32)>);

impl Asset for Day2Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}

fn day1_part2() {
    let input: Vec<u32> = get_input::<Day1Input>("day-1").0;
    let mut averaged = Vec::new();
    for i in 2..input.len() {
        averaged.push(input[i - 2] + input[i - 1] + input[i]);
    }
    let mut count = 0;
    for i in 1..averaged.len() {
        if averaged[i] > averaged[i - 1] {
            count += 1;
        }
    }
    dbg!(count);
}

fn day1_part1() {
    let input: Vec<u32> = get_input::<Day1Input>("day-1").0;
    let mut count = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            count += 1;
        }
    }
    dbg!(count);
}

#[derive(Clone, Deserialize)]
struct Day1Input(Vec<u32>);

impl Asset for Day1Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}

fn get_input<T: assets_manager::Asset>(path: &str) -> T
where
    T: Clone,
{
    let cache = AssetCache::new("assets").unwrap();
    cache.load::<T>(path).unwrap().cloned()
}
