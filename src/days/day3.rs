use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let Input(input) = get_input::<Input>("day-3");
    let mut generator = input.clone();
    let mut i = 0;
    while generator.len() > 1 {
        let sum = generator.iter().map(|num| num[i]).sum::<usize>();
        let filter = if sum >= (generator.len() + 1) / 2 {
            1
        } else {
            0
        };
        generator = generator
            .into_iter()
            .filter(|num| num[i] == filter)
            .collect::<Vec<_>>();
        i += 1;
    }
    let generator = from_bits(&generator[0]);
    let mut scrubber = input;
    let mut i = 0;
    while scrubber.len() > 1 {
        let sum = scrubber.iter().map(|num| num[i]).sum::<usize>();
        let filter = if sum >= (scrubber.len() + 1) / 2 {
            0
        } else {
            1
        };
        scrubber = scrubber
            .into_iter()
            .filter(|num| num[i] == filter)
            .collect::<Vec<_>>();
        i += 1;
    }
    let scrubber = from_bits(&scrubber[0]);
    let life_support = generator * scrubber;
    dbg!(life_support);
}

fn part1() {
    let Input(input) = get_input::<Input>("day-3");
    let mut gamma: VecDeque<_> = vec![0; input[0].len()].into();
    let mut epsilon: VecDeque<_> = vec![1; input[0].len()].into();
    for j in 0..gamma.len() {
        let sum = input.iter().map(|num| num[j]).sum::<usize>();
        if sum > input.len() / 2 {
            gamma[j] = 1;
            epsilon[j] = 0;
        }
    }
    let gamma = from_bits(&gamma);
    let epsilon = from_bits(&epsilon);
    let power_cons = gamma * epsilon;
    dbg!(power_cons);
}

fn from_bits(bits: &VecDeque<usize>) -> usize {
    bits.iter().fold(0, |num, bit| (num << 1) + bit)
}

#[derive(Clone, Deserialize)]
struct Input(Vec<VecDeque<usize>>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
