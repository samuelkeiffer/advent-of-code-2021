use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let Input(input) = get_input::<Input>("day-18");
    let mut biggest_magnitude = 0;
    for (i, left) in input.iter().enumerate() {
        for (j, right) in input.iter().enumerate() {
            if i != j {
                let mut snail_number = left.clone().add(right.clone());
                while snail_number.reduce() {}
                biggest_magnitude = biggest_magnitude.max(snail_number.magnitude());
            }
        }
    }
    dbg!(biggest_magnitude);
}

fn part1() {
    let Input(input) = get_input::<Input>("day-18");
    let mut numbers = input.into_iter();
    let mut snail_number = numbers.next().unwrap();
    for number in numbers {
        snail_number = snail_number.add(number);
        while snail_number.reduce() {}
    }
    dbg!(snail_number.magnitude());
}

impl SnailNumber {
    fn magnitude(&self) -> u32 {
        match self {
            Self::Single(num) => *num,
            Self::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn reduce(&mut self) -> bool {
        if self.explode() {
            true
        } else {
            self.split()
        }
    }

    fn split(&mut self) -> bool {
        let mut split = false;
        let mut vec = self.clone().to_vec();
        if let Some((i, (path, num))) = vec
            .clone()
            .iter()
            .enumerate()
            .find(|(_i, (_path, num))| *num >= 10)
        {
            vec.remove(i);
            let mut path1 = path.clone();
            path1.push_back(true);
            let num1 = num / 2;
            vec.push((path1, num1));
            let mut path2 = path.clone();
            path2.push_back(false);
            let num2 = num - num1;
            vec.push((path2, num2));
            split = true;
        }
        *self = Self::from_vec(vec);
        split
    }

    fn explode(&mut self) -> bool {
        let mut exploded = false;
        let mut vec = self.clone().to_vec();
        for (l, l_path) in vec
            .iter()
            .enumerate()
            .filter_map(|(i, (path, _num))| (path.len() > 4).then_some((i, path)))
        {
            let mut l_path = l_path.clone();
            if let Some(side) = l_path.back_mut() {
                *side = !*side;
            }
            if let Some(r) = vec.iter().enumerate().find_map(|(i, (path, _num))| {
                (l_path.len() == path.len() && l_path.iter().zip(path.iter()).all(|(l, r)| l == r))
                    .then_some(i)
            }) {
                exploded = true;
                if l != 0 {
                    vec[l - 1].1 += vec[l].1;
                }
                let r_val = vec[r].1;
                if let Some(number) = vec.get_mut(r + 1) {
                    number.1 += r_val;
                }
                let mut path = vec[r].0.clone();
                path.pop_back();
                vec.remove(r);
                vec.remove(l);
                vec.push((path, 0));
                break;
            }
        }
        *self = Self::from_vec(vec);
        exploded
    }

    fn from_vec(vec: Vec<(VecDeque<bool>, u32)>) -> Self {
        let mut snail_number = SnailNumber::Single(0);
        for (path, num) in vec {
            fn create_pair(snail_num: &mut SnailNumber, path: &VecDeque<bool>) {
                loop {
                    match snail_num.access_path(path.clone()) {
                        Some(SnailNumber::Pair(_, _)) => break,
                        Some(num @ SnailNumber::Single(_)) => {
                            *num = SnailNumber::Pair(
                                Box::new(SnailNumber::Single(0)),
                                Box::new(SnailNumber::Single(0)),
                            )
                        }
                        None => {
                            let mut path = path.clone();
                            path.pop_back();
                            create_pair(snail_num, &path);
                        }
                    }
                }
            }
            if snail_number.access_path(path.clone()).is_none() {
                let path = path.clone();
                create_pair(&mut snail_number, &path);
            }
            let number = snail_number.access_path(path).unwrap();
            *number = Self::Single(num);
        }
        snail_number
    }

    fn access_path(&mut self, mut path: VecDeque<bool>) -> Option<&mut Self> {
        if let Some(side) = path.pop_front() {
            if let SnailNumber::Pair(left, right) = self {
                if side {
                    left.access_path(path)
                } else {
                    right.access_path(path)
                }
            } else {
                None
            }
        } else {
            Some(self)
        }
    }

    fn to_vec(self) -> Vec<(VecDeque<bool>, u32)> {
        fn push_vec(
            snail_number: &SnailNumber,
            path: VecDeque<bool>,
            vec: &mut Vec<(VecDeque<bool>, u32)>,
        ) {
            match snail_number {
                SnailNumber::Single(num) => vec.push((path, *num)),
                SnailNumber::Pair(num1, num2) => {
                    let mut path1 = path.clone();
                    path1.push_back(true);
                    push_vec(num1, path1, vec);
                    let mut path2 = path;
                    path2.push_back(false);
                    push_vec(num2, path2, vec);
                }
            }
        }

        let mut vec = Vec::new();
        push_vec(&self, VecDeque::new(), &mut vec);
        vec
    }

    fn add(self, other: Self) -> Self {
        Self::Pair(Box::new(self), Box::new(other))
    }
}

#[derive(Debug, Clone, Deserialize)]
enum SnailNumber {
    Single(u32),
    Pair(Box<SnailNumber>, Box<SnailNumber>),
}

#[derive(Clone, Deserialize)]
struct Input(Vec<SnailNumber>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
