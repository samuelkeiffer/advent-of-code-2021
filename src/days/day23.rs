use crate::*;
use core::fmt;

pub fn run() {
    pathfind();
}

fn pathfind() {
    let input = get_input::<Input>("day-23");
    let burrow = Burrow::from_input(input);

    let (_path, cost) = astar(
        &burrow,
        |burrow| burrow.available_states().collect::<Vec<_>>(),
        |burrow| burrow.heuristic(),
        |burrow| burrow.success_state(),
    )
    .unwrap();
    dbg!(cost);
}

impl Burrow {
    fn home_tiles(&self, amphipod: Amphipod) -> impl Iterator<Item = (usize, usize)> + '_ {
        let home_column = match amphipod {
            Amphipod::A => 3,
            Amphipod::B => 5,
            Amphipod::C => 7,
            Amphipod::D => 9,
        };

        self.enumerated_tiles()
            .filter(move |((_row, column), tile)| {
                *column == home_column && tile.kind == TileKind::Room
            })
            .map(|(pos, _tile)| pos)
            .sorted()
            .rev()
    }

    fn path_is_clear(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let path = self.path(from, to);
        path.iter().all(|tile| self.tile(*tile).occupancy.is_none())
    }

    fn dist(&self, from: (usize, usize), to: (usize, usize)) -> usize {
        self.path(from, to).len()
    }

    fn path(&self, from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
        let mut current_pos = from;
        let mut path = Vec::new();
        while current_pos != to {
            match self.tile(current_pos).kind {
                TileKind::Room => {
                    if current_pos.1 == to.1 {
                        if current_pos.0 > to.0 {
                            current_pos.0 -= 1;
                        } else {
                            current_pos.0 += 1;
                        }
                    } else {
                        current_pos.0 -= 1;
                    }
                }
                TileKind::Entrance => {
                    if current_pos.1 == to.1 {
                        current_pos.0 += 1;
                    } else if current_pos.1 > to.1 {
                        current_pos.1 -= 1;
                    } else {
                        current_pos.1 += 1;
                    }
                }
                TileKind::Hallway => {
                    if current_pos.1 > to.1 {
                        current_pos.1 -= 1;
                    } else {
                        current_pos.1 += 1;
                    }
                }
            }
            path.push(current_pos);
        }
        path
    }

    fn amphipod_positions(&self) -> impl Iterator<Item = (Amphipod, (usize, usize))> + '_ {
        self.enumerated_tiles()
            .filter_map(|(pos, tile)| tile.occupancy.map(|amphipod| (amphipod, pos)))
    }

    fn enumerated_tiles(&self) -> impl Iterator<Item = ((usize, usize), &Tile)> {
        self.0.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, tile)| tile.as_ref().map(|tile| ((i, j), tile)))
        })
    }

    fn swapped_occupancies(mut self, a: (usize, usize), b: (usize, usize)) -> Self {
        let a_tile_occupancy = self.0[a.0][a.1].unwrap().occupancy;
        let b_tile_occupancy = self.0[b.0][b.1].unwrap().occupancy;
        if let Some(a_tile) = self.0[a.0][a.1].as_mut() {
            a_tile.occupancy = b_tile_occupancy;
        }
        if let Some(b_tile) = self.0[b.0][b.1].as_mut() {
            b_tile.occupancy = a_tile_occupancy;
        }
        self
    }

    fn tile(&self, pos: (usize, usize)) -> Tile {
        self.0[pos.0][pos.1].unwrap()
    }

    fn available_movements(
        &self,
        origin: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.enumerated_tiles()
            .filter(move |(pos, tile)| {
                let origin_tile = self.tile(origin);
                tile.occupancy.is_none()
                    && ((origin_tile.kind == TileKind::Room && tile.kind != TileKind::Entrance)
                        || (origin_tile.kind == TileKind::Hallway && tile.kind == TileKind::Room))
                    && if tile.kind == TileKind::Room {
                        origin_tile.occupancy.map_or(false, |amphipod| {
                            self.home_tiles(amphipod).any(|home_pos| home_pos == *pos)
                                && self.home_tiles(amphipod).all(|home_pos| {
                                    self.tile(home_pos)
                                        .occupancy
                                        .map_or(true, |amphipod_b| amphipod == amphipod_b)
                                })
                        })
                    } else {
                        true
                    }
                    && self.path_is_clear(origin, *pos)
            })
            .map(|(pos, _tile)| pos)
    }

    fn available_states(&self) -> impl Iterator<Item = (Self, usize)> + '_ {
        self.amphipod_positions()
            .flat_map(move |(amphipod, start_pos)| {
                self.available_movements(start_pos).map(move |pos| {
                    (
                        (*self).swapped_occupancies(start_pos, pos),
                        self.dist(start_pos, pos) * amphipod.cost(),
                    )
                })
            })
    }

    fn heuristic(&self) -> usize {
        self.amphipod_positions()
            .map(|(amphipod, pos)| {
                let home_pos = self
                    .home_tiles(amphipod)
                    .find(|home| self.tile(*home).occupancy != Some(amphipod))
                    .unwrap_or(pos);
                if pos.1 == home_pos.1 && pos.0 > home_pos.0 {
                    0
                } else {
                    let dist = self.dist(pos, home_pos);
                    dist * amphipod.cost()
                }
            })
            .sum::<usize>()
    }

    fn success_state(&self) -> bool {
        let mut positions = BTreeMap::new();
        for row in self.0 {
            for (j, tile) in row.iter().enumerate() {
                if let Some(tile) = tile {
                    let incomplete_movement = if tile.kind == TileKind::Room {
                        tile.occupancy.is_none()
                    } else {
                        tile.occupancy.is_some()
                    };
                    if incomplete_movement {
                        return false;
                    }

                    if let Some(amphipod) = tile.occupancy {
                        let entry = positions.entry(amphipod).or_insert_with(BTreeSet::new);
                        entry.insert(j);
                    }
                }
            }
        }

        let mut last = 0;
        for (_, positions) in positions.iter() {
            if *positions.first().unwrap() < last {
                return false;
            }
            last = *positions.last().unwrap();
        }

        true
    }

    fn from_input(input: Input) -> Self {
        let input = input.0;
        let mut array = [[None; NUM_COLS]; NUM_ROWS];
        for (i, row) in input.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if let Some(tile) = *tile {
                    let kind = if input[i][j - 1].is_none() && input[i][j + 1].is_none() {
                        TileKind::Room
                    } else if input[i + 1][j].is_some() {
                        TileKind::Entrance
                    } else {
                        TileKind::Hallway
                    };
                    let occupancy = match tile {
                        InputTileKind::Empty => None,
                        InputTileKind::A => Some(Amphipod::A),
                        InputTileKind::B => Some(Amphipod::B),
                        InputTileKind::C => Some(Amphipod::C),
                        InputTileKind::D => Some(Amphipod::D),
                    };
                    array[i][j] = Some(Tile { kind, occupancy })
                }
            }
        }
        Burrow(array)
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Burrow([[Option<Tile>; NUM_COLS]; NUM_ROWS]);

impl fmt::Debug for Burrow {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let get_char = |tile| match tile {
            None => '#',
            Some(None) => '.',
            Some(Some(Amphipod::A)) => 'A',
            Some(Some(Amphipod::B)) => 'B',
            Some(Some(Amphipod::C)) => 'C',
            Some(Some(Amphipod::D)) => 'D',
        };
        let strings = self.0.iter().map(|row| {
            row.iter()
                .map(|tile| get_char(tile.map(|tile| tile.occupancy)))
                .collect::<String>()
        });
        fmt.debug_set().entries(strings).finish()
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Tile {
    kind: TileKind,
    occupancy: Option<Amphipod>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum TileKind {
    Hallway,
    Entrance,
    Room,
}

#[derive(Clone, Deserialize, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn cost(self) -> usize {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }
}

#[derive(Clone, Deserialize, Copy, Debug)]
enum InputTileKind {
    Empty,
    A,
    B,
    C,
    D,
}

#[derive(Clone, Deserialize)]
struct Input([[Option<InputTileKind>; NUM_COLS]; NUM_ROWS]);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}

const NUM_ROWS: usize = 7;
const NUM_COLS: usize = 13;
