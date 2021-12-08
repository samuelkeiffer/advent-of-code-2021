use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let input: Vec<(Vec<String>, Vec<String>)> = get_input::<Input>("day-8").0;
    let sum = input
        .iter()
        .map(|(test, output)| {
            let convert_key = |num: &String| num.chars().sorted().rev().collect::<String>();
            let mut map = HashMap::new();
            let one = test.iter().find(|num| num.chars().count() == 2).unwrap();
            map.insert(convert_key(one), 1);
            let four = test.iter().find(|num| num.chars().count() == 4).unwrap();
            map.insert(convert_key(four), 4);
            let seven = test.iter().find(|num| num.chars().count() == 3).unwrap();
            map.insert(convert_key(seven), 7);
            let eight = test.iter().find(|num| num.chars().count() == 7).unwrap();
            map.insert(convert_key(eight), 8);
            let nine = test
                .iter()
                .find(|num| {
                    num.chars().count() == 6
                        && num
                            .chars()
                            .filter(|c| four.contains(&c.to_string()))
                            .count()
                            == 4
                })
                .unwrap();
            map.insert(convert_key(nine), 9);
            let zero = test
                .iter()
                .find(|num| {
                    num.chars().count() == 6
                        && num
                            .chars()
                            .filter(|c| four.contains(&c.to_string()))
                            .count()
                            == 3
                        && num.chars().filter(|c| one.contains(&c.to_string())).count() == 2
                })
                .unwrap();
            map.insert(convert_key(zero), 0);
            let six = test
                .iter()
                .find(|num| {
                    num.chars().count() == 6
                        && num
                            .chars()
                            .filter(|c| four.contains(&c.to_string()))
                            .count()
                            == 3
                        && num.chars().filter(|c| one.contains(&c.to_string())).count() == 1
                })
                .unwrap();
            map.insert(convert_key(six), 6);
            let three = test
                .iter()
                .find(|num| {
                    num.chars().count() == 5
                        && num.chars().filter(|c| one.contains(&c.to_string())).count() == 2
                })
                .unwrap();
            map.insert(convert_key(three), 3);
            let two = test
                .iter()
                .find(|num| {
                    num.chars().count() == 5
                        && num
                            .chars()
                            .filter(|c| four.contains(&c.to_string()))
                            .count()
                            == 2
                })
                .unwrap();
            map.insert(convert_key(two), 2);
            let five = test
                .iter()
                .find(|num| {
                    num.chars().count() == 5
                        && num
                            .chars()
                            .filter(|c| four.contains(&c.to_string()))
                            .count()
                            == 3
                        && num.chars().filter(|c| one.contains(&c.to_string())).count() == 1
                })
                .unwrap();
            map.insert(convert_key(five), 5);

            output
                .iter()
                .map(|out| {
                    let key = out.chars().sorted().rev().collect::<String>();
                    map.get(&key).unwrap()
                })
                .fold(0, |a, b| a * 10 + b)
        })
        .sum::<u32>();
    dbg!(sum);
}

fn part1() {
    let input: Vec<(Vec<String>, Vec<String>)> = get_input::<Input>("day-8").0;
    const DIGITS: [usize; 4] = [2, 4, 3, 7];
    let unique = input
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .map(|num| {
                    let segments = num.chars().count();
                    if DIGITS.contains(&segments) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();
    dbg!(unique);
}

#[derive(Clone, Deserialize)]
struct Input(Vec<(Vec<String>, Vec<String>)>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
