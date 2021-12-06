use crate::*;

pub fn run() {
    part1();
    part2();
}

pub fn part2() {
    let input: Vec<u32> = get_input::<Input>("day-1").0;
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

fn part1() {
    let input: Vec<u32> = get_input::<Input>("day-1").0;
    let mut count = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            count += 1;
        }
    }
    dbg!(count);
}

#[derive(Clone, Deserialize)]
struct Input(Vec<u32>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
