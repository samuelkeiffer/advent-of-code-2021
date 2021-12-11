use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let input: Vec<Vec<u32>> = get_input::<Input>("day-11").0;
    let length = input.len();
    let width = input[0].len();
    let mut octopi = HashMap::new();
    let mut all_flash = 0;
    for i in 0..length {
        for j in 0..width {
            octopi.insert((i as i32, j as i32), (input[i][j], false));
        }
    }
    for i in 0..1000 {
        let mut flashes = 0;
        octopi.iter_mut().for_each(|(_, (power, _))| *power += 1);
        while octopi
            .iter()
            .any(|(_, (power, flashed))| !flashed && *power > 9)
        {
            flash(&mut octopi);
            flashes += 1;
        }
        octopi
            .iter_mut()
            .filter(|(_, (_, flashed))| *flashed)
            .for_each(|(_, (power, flashed))| {
                *power = 0;
                *flashed = false;
            });
        if flashes == 100 {
            all_flash = i + 1;
            break;
        }
    }
    dbg!(all_flash);
}

fn part1() {
    let input: Vec<Vec<u32>> = get_input::<Input>("day-11").0;
    let length = input.len();
    let width = input[0].len();
    let mut octopi = HashMap::new();
    let mut flashes = 0;
    for i in 0..length {
        for j in 0..width {
            octopi.insert((i as i32, j as i32), (input[i][j], false));
        }
    }
    for _ in 0..100 {
        octopi.iter_mut().for_each(|(_, (power, _))| *power += 1);
        while octopi
            .iter()
            .any(|(_, (power, flashed))| !flashed && *power > 9)
        {
            flash(&mut octopi);
            flashes += 1;
        }
        octopi
            .iter_mut()
            .filter(|(_, (_, flashed))| *flashed)
            .for_each(|(_, (power, flashed))| {
                *power = 0;
                *flashed = false;
            });
    }
    dbg!(flashes);
}

fn flash(octopi: &mut HashMap<(i32, i32), (u32, bool)>) {
    let coords = if let Some(((x, y), (_, flashed))) = octopi
        .iter_mut()
        .find(|(_, (power, flashed))| !flashed && *power > 9)
    {
        *flashed = true;
        Some((*x, *y))
    } else {
        None
    };
    if let Some((x, y)) = coords {
        let x = x as i32;
        let y = y as i32;
        for x in (x - 1)..=(x + 1) {
            for y in (y - 1)..=(y + 1) {
                if let Some((power, _)) = octopi.get_mut(&(x, y)) {
                    *power += 1;
                }
            }
        }
    }
}

#[derive(Clone, Deserialize)]
struct Input(Vec<Vec<u32>>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
