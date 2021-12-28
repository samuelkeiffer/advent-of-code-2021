use crate::*;

pub fn run() {
    part1();
}

fn part1() {
    let mut region = get_input::<Region>("day-25");
    let mut steps = 1;
    while region.step() {
        steps += 1;
    }
    dbg!(steps);
}

impl Region {
    fn step(&mut self) -> bool {
        let mut right_steps = Vec::new();
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                if let Some(Cucumber::Right) = self.0[i][j] {
                    let j2 = if (j + 1) == self.0[0].len() { 0 } else { j + 1 };
                    if self.0[i][j2].is_none() {
                        right_steps.push(((i, j), (i, j2)));
                    }
                }
            }
        }
        for (from, to) in &right_steps {
            self.swap(*from, *to);
        }

        let mut down_steps = Vec::new();
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                if let Some(Cucumber::Down) = self.0[i][j] {
                    let i2 = if (i + 1) == self.0.len() { 0 } else { i + 1 };
                    if self.0[i2][j].is_none() {
                        down_steps.push(((i, j), (i2, j)));
                    }
                }
            }
        }
        for (from, to) in &down_steps {
            self.swap(*from, *to);
        }

        !right_steps.is_empty() || !down_steps.is_empty()
    }

    fn swap(&mut self, (i, j): (usize, usize), (i2, j2): (usize, usize)) {
        let cuc1 = self.0[i][j];
        self.0[i][j] = self.0[i2][j2];
        self.0[i2][j2] = cuc1;
    }
}

#[derive(Clone, Deserialize, Copy)]
enum Cucumber {
    Right,
    Down,
}

#[derive(Clone, Deserialize)]
struct Region(Vec<Vec<Option<Cucumber>>>);

impl Asset for Region {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
