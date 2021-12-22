use crate::*;

pub fn run() {
    part1();
    part2();
}

// 2758514936282235 for test
fn part2() {
    let Input(startup) = get_input::<Input>("day-22");
}

fn part1() {
    let Input(startup) = get_input::<Input>("day-22");
    let startup_rev = startup.into_iter().map(|(state, min, max)| {
        (state, Aabb { min, max })
    })
    .rev()
    .collect::<Vec<_>>();

    let mut count = 0;

    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                let pos = Vec3::new(x, y, z);
                if let Some(state) = startup_rev.iter().find_map(|(state, aabb)| aabb.contains_point(pos).then_some(state)) {
                    if *state {
                        count += 1;
                    }
                }
            }
        }
    }
    dbg!(count);
}

#[derive(Clone, Deserialize)]
struct Input(Vec<(bool, Vec3<i128>, Vec3<i128>)>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
