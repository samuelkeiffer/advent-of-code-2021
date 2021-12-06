use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let input: Vec<_> = get_input::<Input>("day-").0;
}

fn part1() {
    let input: Vec<_> = get_input::<Input>("day-").0;
}

#[derive(Clone, Deserialize)]
struct Input(Vec<u32>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
