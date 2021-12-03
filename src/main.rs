#![allow(dead_code)]
use assets_manager::{loader::RonLoader, Asset, AssetCache};
use serde::Deserialize;

fn main() {
    day2_part2();
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
