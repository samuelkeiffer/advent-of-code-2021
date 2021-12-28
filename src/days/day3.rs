use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {}

fn part1() {
    let Input(input) = get_input::<Input>("day-3-test");
}

#[derive(Clone, Deserialize)]
struct Input(Vec<u32>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
