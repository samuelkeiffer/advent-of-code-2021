use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let input: Vec<((u32, u32), (u32, u32))> = get_input::<Input>("day-5").0;
    let vent_spots = input
        .iter()
        .flat_map(|((x1, y1), (x2, y2))| {
            if x1 == x2 {
                if y1 > y2 {
                    (*y2..=*y1)
                        .into_iter()
                        .map(|y| (*x1, y))
                        .collect::<Vec<_>>()
                } else {
                    (*y1..=*y2)
                        .into_iter()
                        .map(|y| (*x1, y))
                        .collect::<Vec<_>>()
                }
            } else if y1 == y2 {
                if x1 > x2 {
                    (*x2..=*x1)
                        .into_iter()
                        .map(|x| (x, *y1))
                        .collect::<Vec<_>>()
                } else {
                    (*x1..=*x2)
                        .into_iter()
                        .map(|x| (x, *y1))
                        .collect::<Vec<_>>()
                }
            } else {
                let x_range = if x1 > x2 {
                    (*x2..=*x1).into_iter().rev().collect::<Vec<_>>()
                } else {
                    (*x1..=*x2).into_iter().collect::<Vec<_>>()
                };
                let y_range = if y1 > y2 {
                    (*y2..=*y1).into_iter().rev().collect::<Vec<_>>()
                } else {
                    (*y1..=*y2).into_iter().collect::<Vec<_>>()
                };
                x_range
                    .into_iter()
                    .zip(y_range.into_iter())
                    .collect::<Vec<_>>()
            }
        })
        .collect::<Vec<(u32, u32)>>();
    let mut count = HashMap::<(u32, u32), u32>::new();
    for spot in vent_spots {
        let entry = count.entry(spot).or_insert(0);
        *entry += 1;
    }
    let number = count
        .iter()
        .filter(|(_, num)| **num > 1)
        .collect::<Vec<_>>()
        .len();
    dbg!(number);
}

fn part1() {
    let input: Vec<((u32, u32), (u32, u32))> = get_input::<Input>("day-5").0;
    let vent_spots = input
        .iter()
        .flat_map(|((x1, y1), (x2, y2))| {
            if x1 == x2 {
                if y1 > y2 {
                    (*y2..=*y1)
                        .into_iter()
                        .map(|y| (*x1, y))
                        .collect::<Vec<_>>()
                } else {
                    (*y1..=*y2)
                        .into_iter()
                        .map(|y| (*x1, y))
                        .collect::<Vec<_>>()
                }
            } else if y1 == y2 {
                if x1 > x2 {
                    (*x2..=*x1)
                        .into_iter()
                        .map(|x| (x, *y1))
                        .collect::<Vec<_>>()
                } else {
                    (*x1..=*x2)
                        .into_iter()
                        .map(|x| (x, *y1))
                        .collect::<Vec<_>>()
                }
            } else {
                Vec::new()
            }
        })
        .collect::<Vec<(u32, u32)>>();
    let mut count = HashMap::<(u32, u32), u32>::new();
    for spot in vent_spots {
        let entry = count.entry(spot).or_insert(0);
        *entry += 1;
    }
    let number = count
        .iter()
        .filter(|(_, num)| **num > 1)
        .collect::<Vec<_>>()
        .len();
    dbg!(number);
}

#[derive(Clone, Deserialize)]
struct Input(Vec<((u32, u32), (u32, u32))>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
