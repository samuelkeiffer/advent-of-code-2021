use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let Input(enhance, map) = get_input::<Input>("day-20");
    let mut map = Map { map, outer: false };
    for _ in 0..50 {
        map = map.enhanced(&enhance);
    }
    let count: usize = map
        .map
        .iter()
        .map(|row| row.iter().filter(|a| **a).count())
        .sum();
    dbg!(count);
}

fn part1() {
    let Input(enhance, map) = get_input::<Input>("day-20");
    let mut map = Map { map, outer: false };
    for _ in 0..2 {
        map = map.enhanced(&enhance);
    }
    let count: usize = map
        .map
        .iter()
        .map(|row| row.iter().filter(|a| **a).count())
        .sum();
    dbg!(count);
}

impl Map {
    fn enhanced(mut self, enhancement: &[bool]) -> Self {
        self = self.padded_3();
        let old = self.clone();
        for i in 1..(old.map.len() - 1) {
            for j in 1..(old.map.len() - 1) {
                let input = [
                    old.map[i - 1][j - 1],
                    old.map[i - 1][j],
                    old.map[i - 1][j + 1],
                    old.map[i][j - 1],
                    old.map[i][j],
                    old.map[i][j + 1],
                    old.map[i + 1][j - 1],
                    old.map[i + 1][j],
                    old.map[i + 1][j + 1],
                ];
                let enhance_index = input
                    .iter()
                    .fold(0, |index, bit| (index << 1) + if *bit { 1 } else { 0 });
                self.map[i][j] = enhancement[enhance_index];
            }
        }
        self.outer = self.map[1][1];
        self = self.lopped_1();
        self
    }

    fn lopped_1(mut self) -> Self {
        self.map.iter_mut().for_each(|col| {
            col.pop_front();
            col.pop_back();
        });
        self.map.pop_front();
        self.map.pop_back();
        self
    }

    fn padded_3(mut self) -> Self {
        let outer = self.outer;
        self.map.iter_mut().for_each(|col| {
            for _ in 0..3 {
                col.push_front(outer);
                col.push_back(outer);
            }
        });
        let col_len = self.map[0].len();
        let mut outer_col = VecDeque::new();
        outer_col.resize(col_len, self.outer);
        for _ in 0..3 {
            self.map.push_front(outer_col.clone());
            self.map.push_back(outer_col.clone());
        }
        self
    }
}

#[derive(Clone, Debug)]
struct Map {
    map: VecDeque<VecDeque<bool>>,
    outer: bool,
}

#[derive(Clone, Deserialize)]
struct Input(Vec<bool>, VecDeque<VecDeque<bool>>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
