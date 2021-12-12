use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let input: Vec<String> = get_input::<Input>("day-10").0;
    let mut errors = Vec::new();
    for (i, line) in input.iter().enumerate() {
        let mut open_chars = Vec::new();
        for a in line.chars() {
            if is_open(a) {
                open_chars.push(a);
            } else {
                let last_open = open_chars.pop().unwrap();
                if a != closing_char(last_open) {
                    errors.push(i);
                }
            }
        }
    }
    let incompletes = input
        .iter()
        .enumerate()
        .filter_map(|(i, line)| (!errors.contains(&i)).then_some(line))
        .collect::<Vec<_>>();
    let points = incompletes
        .iter()
        .map(|line| {
            let mut open_chars = Vec::new();
            for a in line.chars() {
                if is_open(a) {
                    open_chars.push(a);
                } else {
                    let last_open = open_chars.pop().unwrap();
                    if a != closing_char(last_open) {
                        panic!("Yeet");
                    }
                }
            }
            let mut points = 0;
            for a in open_chars.iter().rev() {
                points *= 5;
                points += close_points(closing_char(*a));
            }
            points
        })
        .sorted()
        .collect::<Vec<_>>();
    dbg!(points[(points.len() - 1) / 2]);
}

fn part1() {
    let input: Vec<String> = get_input::<Input>("day-10").0;
    let mut errors = Vec::new();
    for line in input {
        let mut open_chars = Vec::new();
        for a in line.chars() {
            if is_open(a) {
                open_chars.push(a);
            } else {
                let last_open = open_chars.pop().unwrap();
                if a != closing_char(last_open) {
                    errors.push(a);
                }
            }
        }
    }
    let sum = errors.iter().map(|a| error_points(*a)).sum::<u32>();
    dbg!(sum);
}

fn closing_char(a: char) -> char {
    match a {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Yeet"),
    }
}

fn is_open(a: char) -> bool {
    matches!(a, '(' | '[' | '{' | '<')
}

fn error_points(a: char) -> u32 {
    match a {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Yeet"),
    }
}

fn close_points(a: char) -> u64 {
    match a {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Yeet"),
    }
}

#[derive(Clone, Deserialize)]
struct Input(Vec<String>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
