use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let input: Vec<i32> = get_input::<Input>("day-7").0;
    let max = input.iter().fold(0, |a, b| a.max(*b));
    let mut min_fuel = i32::MAX;
    for i in 0..max {
        let fuel = input
            .iter()
            .map(|a| {
                let dist = (a - i).abs();
                (dist * (dist + 1)) / 2
            })
            .sum::<i32>();
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    dbg!(min_fuel);
}

fn part1() {
    let input: Vec<i32> = get_input::<Input>("day-7").0;
    let max = input.iter().fold(0, |a, b| a.max(*b));
    let mut min_fuel = i32::MAX;
    for i in 0..max {
        let fuel = input.iter().map(|a| (a - i).abs()).sum::<i32>();
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    dbg!(min_fuel);
}

#[derive(Clone, Deserialize)]
struct Input(Vec<i32>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
