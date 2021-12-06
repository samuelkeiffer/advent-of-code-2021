use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let input: Vec<u64> = get_input::<Input>("day-6").0;
    let mut fish = BTreeMap::new();
    for a in input.iter() {
        let entry = fish.entry(a).or_insert(0);
        *entry += 1;
    }
    for i in 0..256 {
        let mut new_fish = BTreeMap::new();
        for (&time, fishes) in fish.iter() {
            if *time > 0 {
                let new_time = match *time {
                    1 => &0,
                    2 => &1,
                    3 => &2,
                    4 => &3,
                    5 => &4,
                    6 => &5,
                    7 => &6,
                    8 => &7,
                    _ => &8,
                };
                let entry = new_fish.entry(new_time).or_insert(0);
                *entry += *fishes;
            } else {
                let entry_a = new_fish.entry(&6).or_insert(0);
                *entry_a += fishes;
                let entry_b = new_fish.entry(&8).or_insert(0);
                *entry_b += fishes;
            }
        }
        fish = new_fish;
    }
    let num_fish = fish.iter().map(|(_, fish)| fish).sum::<u64>();
    dbg!(num_fish);
}

fn part1() {
    let mut fish: Vec<u64> = get_input::<Input>("day-6").0;
    for i in 0..80 {
        let mut new_fish = Vec::new();
        fish.iter_mut().for_each(|fish| {
            if *fish > 0 {
                *fish -= 1;
            } else {
                *fish = 6;
                new_fish.push(8);
            }
        });
        fish.append(&mut new_fish);
    }
    dbg!(fish.len());
}

#[derive(Clone, Deserialize)]
struct Input(Vec<u64>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
