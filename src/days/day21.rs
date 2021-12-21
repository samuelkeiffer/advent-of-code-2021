use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let player_1 = QuantumPlayer::new(PLAYER_1_POS);
    let player_2 = QuantumPlayer::new(PLAYER_2_POS);
    let mut universes = HashMap::new();
    universes.insert(Universe { player_1, player_2 }, 1);
    let mut player_1_wins: u128 = 0;
    let mut player_2_wins: u128 = 0;
    let mut player_1_turn = true;
    loop {
        if universes.is_empty() {
            break;
        }
        let mut new_universes = HashMap::new();
        for (universe, number) in universes.iter() {
            if let Some(player_1_winner) = universe.winner() {
                if player_1_winner {
                    player_1_wins += number;
                } else {
                    player_2_wins += number;
                }
            } else {
                let add_universes = universe.player_turn(player_1_turn);
                for universe in add_universes {
                    let entry = new_universes.entry(universe).or_insert(0);
                    *entry += number;
                }
            }
        }
        player_1_turn = !player_1_turn;
        universes = new_universes;
    }
    dbg!(player_1_wins.max(player_2_wins));
}

impl Universe {
    fn player_turn(self, player_1: bool) -> Vec<Universe> {
        if player_1 {
            DIRAC_ROLL
                .iter()
                .flat_map(|roll1| {
                    DIRAC_ROLL.iter().flat_map(move |roll2| {
                        DIRAC_ROLL.iter().map(move |roll3| roll1 + roll2 + roll3)
                    })
                })
                .map(|roll| {
                    let mut universe = self;
                    universe.player_1.take_turn(roll);
                    universe
                })
                .collect::<Vec<_>>()
        } else {
            DIRAC_ROLL
                .iter()
                .flat_map(|roll1| {
                    DIRAC_ROLL.iter().flat_map(move |roll2| {
                        DIRAC_ROLL.iter().map(move |roll3| roll1 + roll2 + roll3)
                    })
                })
                .map(|roll| {
                    let mut universe = self;
                    universe.player_2.take_turn(roll);
                    universe
                })
                .collect::<Vec<_>>()
        }
    }

    fn winner(&self) -> Option<bool> {
        if self.player_1.won() {
            Some(true)
        } else if self.player_2.won() {
            Some(false)
        } else {
            None
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq, Copy)]
struct Universe {
    player_1: QuantumPlayer,
    player_2: QuantumPlayer,
}

impl QuantumPlayer {
    fn won(&self) -> bool {
        self.score >= 21
    }

    fn take_turn(&mut self, roll: u32) {
        let mut end = self.pos + roll;
        while end > 10 {
            end -= 10;
        }
        self.pos = end;
        self.score += end;
    }

    fn new(start: u32) -> Self {
        Self {
            pos: start,
            score: 0,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq, Copy)]
struct QuantumPlayer {
    pos: u32,
    score: u32,
}

const DIRAC_ROLL: [u32; 3] = [1, 2, 3];

fn part1() {
    let mut die = DeterministicDie::new();
    let mut player_1 = Player::new(PLAYER_1_POS);
    let mut player_2 = Player::new(PLAYER_2_POS);
    loop {
        player_1.take_turn(&mut die);
        if player_1.won() {
            break;
        }
        player_2.take_turn(&mut die);
        if player_2.won() {
            break;
        }
    }
    let losing_score = if player_1.won() {
        player_2.score
    } else {
        player_1.score
    };
    dbg!(losing_score * die.rolled);
}

impl Player {
    fn won(&self) -> bool {
        self.score >= 1000
    }

    fn take_turn(&mut self, die: &mut DeterministicDie) {
        let mut result = 0;
        for _ in 0..3 {
            result += die.roll();
        }
        let mut end = self.pos + result;
        while end > 10 {
            end -= 10;
        }
        self.pos = end;
        self.score += end;
    }

    fn new(start: u32) -> Self {
        Self {
            pos: start,
            score: 0,
        }
    }
}

struct Player {
    pos: u32,
    score: u32,
}

impl DeterministicDie {
    fn new() -> Self {
        Self { val: 1, rolled: 0 }
    }

    fn roll(&mut self) -> u32 {
        let roll = self.val;
        self.val += 1;
        if self.val > 100 {
            self.val = 1;
        }
        self.rolled += 1;
        roll
    }
}

struct DeterministicDie {
    val: u32,
    rolled: u32,
}

// Real
const PLAYER_1_POS: u32 = 1;
const PLAYER_2_POS: u32 = 5;

// Test
// const PLAYER_1_POS: u32 = 4;
// const PLAYER_2_POS: u32 = 8;
