use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
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

fn part1() {
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