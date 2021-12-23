use crate::*;

pub fn run() {
    part1();
    part2();
}

// 2758514936282235 for test
fn part2() {
    let Input(startup) = get_input::<Input>("day-22");
    let startup_rev = startup
        .into_iter()
        .map(|(state, min, max)| (state, Aabb { min, max }))
        .rev()
        .collect::<Vec<_>>();

    let mut count: i128 = 0;
    let mut checked_aabbs = Vec::new();

    for (state, aabb) in startup_rev.iter() {
        if *state {
            fn unknown_in_aabb(aabb: Aabb<i128>, checked_aabbs: &[Aabb<i128>]) -> i128 {
                if !checked_aabbs
                    .iter()
                    .any(|checked_aabb| aabbs_collide(*checked_aabb, aabb))
                {
                    aabb_size(aabb)
                } else if checked_aabbs
                    .iter()
                    .any(|checked_aabb| checked_aabb.contains_aabb(aabb))
                {
                    0
                } else {
                    let checked_aabb = checked_aabbs
                        .iter()
                        .find(|checked_aabb| aabbs_collide(**checked_aabb, aabb))
                        .unwrap();
                    let intersection = checked_aabb.intersection(aabb);
                    let x_split =
                        !(aabb.max.x == intersection.max.x && aabb.min.x == intersection.min.x);
                    let y_split =
                        !(aabb.max.y == intersection.max.y && aabb.min.y == intersection.min.y);
                    let z_split =
                        !(aabb.max.z == intersection.max.z && aabb.min.x == intersection.min.z);
                    if x_split {
                        let x_split = if aabb.min.x == intersection.min.x {
                            intersection.max.x + 1
                        } else {
                            intersection.min.x
                        };
                        let left = Aabb {
                            min: aabb.min,
                            max: Vec3::new(x_split - 1, aabb.max.y, aabb.max.z),
                        };
                        let right = Aabb {
                            min: Vec3::new(x_split, aabb.min.y, aabb.min.z),
                            max: aabb.max,
                        };
                        unknown_in_aabb(left, checked_aabbs) + unknown_in_aabb(right, checked_aabbs)
                    } else if y_split {
                        let y_split = if aabb.min.y == intersection.min.y {
                            intersection.max.y + 1
                        } else {
                            intersection.min.y
                        };
                        let left = Aabb {
                            min: aabb.min,
                            max: Vec3::new(aabb.max.x, y_split - 1, aabb.max.z),
                        };
                        let right = Aabb {
                            min: Vec3::new(aabb.min.x, y_split, aabb.min.z),
                            max: aabb.max,
                        };
                        unknown_in_aabb(left, checked_aabbs) + unknown_in_aabb(right, checked_aabbs)
                    } else if z_split {
                        let z_split = if aabb.min.z == intersection.min.z {
                            intersection.max.z + 1
                        } else {
                            intersection.min.z
                        };
                        let left = Aabb {
                            min: aabb.min,
                            max: Vec3::new(aabb.max.x, aabb.max.y, z_split - 1),
                        };
                        let right = Aabb {
                            min: Vec3::new(aabb.min.x, aabb.min.y, z_split),
                            max: aabb.max,
                        };
                        unknown_in_aabb(left, checked_aabbs) + unknown_in_aabb(right, checked_aabbs)
                    } else {
                        panic!();
                    }
                }
            }

            count += unknown_in_aabb(*aabb, &checked_aabbs);
        }
        checked_aabbs.push(*aabb);
    }

    dbg!(count);
}

fn aabb_size(aabb: Aabb<i128>) -> i128 {
    aabb.size().map(|x| x + 1).product()
}

fn aabbs_collide(a: Aabb<i128>, b: Aabb<i128>) -> bool {
    let a = Aabb {
        min: a.min.map(|a| a - 1),
        max: a.max.map(|a| a + 1),
    };
    a.collides_with_aabb(b)
}

fn part1() {
    let Input(startup) = get_input::<Input>("day-22");
    let startup_rev = startup
        .into_iter()
        .map(|(state, min, max)| (state, Aabb { min, max }))
        .rev()
        .collect::<Vec<_>>();

    let mut count = 0;

    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                let pos = Vec3::new(x, y, z);
                if let Some(state) = startup_rev
                    .iter()
                    .find_map(|(state, aabb)| aabb.contains_point(pos).then_some(state))
                {
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
