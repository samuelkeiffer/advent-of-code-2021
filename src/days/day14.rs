use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let Input(template, initial_chain) = get_input::<Input>("day-14");
    let new_template = template
        .into_iter()
        .map(|(key, value)| {
            let char_1 = key.chars().next().unwrap();
            let char_2 = key.chars().nth(1).unwrap();
            ((char_1, char_2), ((char_1, value), (value, char_2)))
        })
        .collect::<HashMap<(char, char), ((char, char), (char, char))>>();
    let mut pairs = HashMap::new();
    for (i, a) in initial_chain.chars().enumerate() {
        if let Some(b) = initial_chain.chars().nth(i + 1) {
            let entry = pairs.entry((a, b)).or_insert(0);
            *entry += 1;
        }
    }
    for _ in 0..40 {
        pairs = grow_chain2(&pairs, &new_template);
    }
    let mut char_count = HashMap::new();
    for ((first, _last), amount) in pairs.iter() {
        let entry = char_count.entry(first).or_insert(0);
        *entry += amount;
    }
    let last_char = initial_chain.chars().last().unwrap();
    let entry = char_count.entry(&last_char).or_insert(0);
    *entry += 1;
    let high = char_count.values().reduce(|a, b| a.max(b)).unwrap();
    let low = char_count.values().reduce(|a, b| a.min(b)).unwrap();
    let diff = high - low;
    dbg!(diff);
}

fn part1() {
    let Input(template, mut chain) = get_input::<Input>("day-14");
    for _ in 0..10 {
        chain = grow_chain(&chain, &template);
    }
    let mut char_count = HashMap::new();
    for a in chain.chars() {
        let entry = char_count.entry(a).or_insert(0);
        *entry += 1;
    }
    let high = char_count.values().reduce(|a, b| a.max(b)).unwrap();
    let low = char_count.values().reduce(|a, b| a.min(b)).unwrap();
    let diff = high - low;
    dbg!(diff);
}

fn grow_chain(starting_chain: &str, template: &HashMap<String, char>) -> String {
    let mut new_chain = String::new();
    for (i, a) in starting_chain.chars().enumerate() {
        new_chain.push(a);
        if let Some(b) = starting_chain.chars().nth(i + 1) {
            if let Some(c) = template.get(&[a, b].iter().collect::<String>()) {
                new_chain.push(*c);
            }
        }
    }
    new_chain
}

fn grow_chain2(
    intial_pairs: &HashMap<(char, char), usize>,
    template: &HashMap<(char, char), ((char, char), (char, char))>,
) -> HashMap<(char, char), usize> {
    let mut final_pairs = HashMap::new();
    for (pair, amount) in intial_pairs.iter() {
        let (new_pair_1, new_pair_2) = template.get(pair).unwrap();
        let entry_1 = final_pairs.entry(*new_pair_1).or_insert(0);
        *entry_1 += amount;
        let entry_2 = final_pairs.entry(*new_pair_2).or_insert(0);
        *entry_2 += amount;
    }
    final_pairs
}

#[derive(Clone, Deserialize)]
struct Input(HashMap<String, char>, String);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
