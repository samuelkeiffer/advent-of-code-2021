use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {}

fn part1() {}

#[derive(Clone, Deserialize)]
struct Input(Vec<u32>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
