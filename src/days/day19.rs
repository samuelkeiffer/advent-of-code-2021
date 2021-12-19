use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let Input(input) = get_input::<Input>("day-19");
    let input = input.into_iter().rev().collect::<Vec<_>>();
    let mut beacon_list = HashSet::new();
    // Relative to reference scanner
    let mut scanner_abs_pos_rot = HashMap::new();
    let mut reference_scanner = None;
    dbg!(input.len());
    for _ in 0..3 {
        if input.len() == scanner_abs_pos_rot.len() {
            break;
        }
        dbg!(scanner_abs_pos_rot.len());
        dbg!(beacon_list.len());
        for (i, scanner_1) in input.iter().enumerate() {
            for (j, scanner_2) in input.iter().enumerate() {
                if i != j
                    && !(scanner_abs_pos_rot.contains_key(&i)
                        && scanner_abs_pos_rot.contains_key(&j))
                {
                    if let Some((beac_1, rot_2_to_1, beac_2)) =
                        overlapping_beacons(scanner_1, scanner_2)
                    {
                        if reference_scanner.is_none() {
                            reference_scanner = Some(i);
                            scanner_abs_pos_rot.insert(i, (Vec3::zero(), Mat3::identity()));
                            beacon_list.extend(scanner_1);
                        }
                        if !scanner_abs_pos_rot.contains_key(&i) {
                            if let Some((abs_pos_2, abs_rot_2)) = scanner_abs_pos_rot.get(&j) {
                                let abs_rot_1 = *abs_rot_2 * rot_2_to_1;
                                let abs_beac_pos =
                                    scanner_2[beac_2] * abs_rot_2.transposed() + abs_pos_2;
                                let abs_pos_1 =
                                    abs_beac_pos - (scanner_1[beac_1] * abs_rot_1.transposed());
                                scanner_abs_pos_rot.insert(i, (abs_pos_1, abs_rot_1));
                                beacon_list.extend(
                                    scanner_1
                                        .iter()
                                        .map(|pos| *pos * abs_rot_1.transposed() + abs_pos_1),
                                );
                                // let test = scanner_1.iter().map(|pos| *pos * abs_rot_1.transposed() + abs_pos_1).collect::<Vec<_>>();
                                // dbg!(i);
                                // dbg!(test);
                            }
                        }
                        if !scanner_abs_pos_rot.contains_key(&j) {
                            if let Some((abs_pos_1, abs_rot_1)) = scanner_abs_pos_rot.get(&i) {
                                let abs_rot_2 = *abs_rot_1 * rot_2_to_1.transposed();
                                let abs_beac_pos =
                                    scanner_1[beac_1] * abs_rot_1.transposed() + abs_pos_1;
                                let abs_pos_2 =
                                    abs_beac_pos - (scanner_2[beac_2] * abs_rot_2.transposed());
                                scanner_abs_pos_rot.insert(j, (abs_pos_2, abs_rot_2));
                                beacon_list.extend(
                                    scanner_2
                                        .iter()
                                        .map(|pos| *pos * abs_rot_2.transposed() + abs_pos_2),
                                );
                                // let test = scanner_2.iter().map(|pos| *pos * abs_rot_2.transposed() + abs_pos_2).collect::<Vec<_>>();
                                // dbg!(j);
                                // dbg!(test);
                            }
                        }
                    }
                }
            }
        }
    }
    for _ in 0..5 {
        dbg!(scanner_abs_pos_rot.len());
        dbg!(beacon_list.len());
        for i in 1..input.len() {
            if !scanner_abs_pos_rot.contains_key(&i) {
                let scanner_2 = &input[i];
                let scanner_1 = beacon_list.iter().copied().collect::<Vec<_>>();
                if let Some((beac_1, rot_2_to_1, beac_2)) =
                    overlapping_beacons(&scanner_1, scanner_2)
                {
                    if !scanner_abs_pos_rot.contains_key(&i) {
                        if let Some((abs_pos_1, abs_rot_1)) = scanner_abs_pos_rot.get(&0) {
                            let abs_rot_2 = *abs_rot_1 * rot_2_to_1.transposed();
                            let abs_beac_pos =
                                scanner_1[beac_1] * abs_rot_1.transposed() + abs_pos_1;
                            let abs_pos_2 =
                                abs_beac_pos - (scanner_2[beac_2] * abs_rot_2.transposed());
                            scanner_abs_pos_rot.insert(i, (abs_pos_2, abs_rot_2));
                            beacon_list.extend(
                                scanner_2
                                    .iter()
                                    .map(|pos| *pos * abs_rot_2.transposed() + abs_pos_2),
                            );
                            // let test = scanner_2.iter().map(|pos| *pos * abs_rot_2.transposed() + abs_pos_2).collect::<Vec<_>>();
                            // dbg!(j);
                            // dbg!(test);
                        }
                    }
                }
            }
        }
    }
    // dbg!(scanner_abs_pos_rot);
    // dbg!(beacon_list.len());
    let mut largest_man_dist = 0;
    for (_i, (pos_a, _rot_a)) in scanner_abs_pos_rot.iter() {
        for (_j, (pos_b, _rot_b)) in scanner_abs_pos_rot.iter() {
            let man_dist =
                (pos_a.x - pos_b.x).abs() + (pos_a.y - pos_b.y).abs() + (pos_a.z - pos_b.z).abs();
            largest_man_dist = man_dist.max(largest_man_dist);
        }
    }
    dbg!(largest_man_dist);
}

fn part1() {
    let Input(input) = get_input::<Input>("day-19");
    let input = input.into_iter().rev().collect::<Vec<_>>();
    let mut beacon_list = HashSet::new();
    // Relative to reference scanner
    let mut scanner_abs_pos_rot = HashMap::new();
    let mut reference_scanner = None;
    dbg!(input.len());
    for _ in 0..3 {
        if input.len() == scanner_abs_pos_rot.len() {
            break;
        }
        dbg!(scanner_abs_pos_rot.len());
        dbg!(beacon_list.len());
        for (i, scanner_1) in input.iter().enumerate() {
            for (j, scanner_2) in input.iter().enumerate() {
                if i != j
                    && !(scanner_abs_pos_rot.contains_key(&i)
                        && scanner_abs_pos_rot.contains_key(&j))
                {
                    if let Some((beac_1, rot_2_to_1, beac_2)) =
                        overlapping_beacons(scanner_1, scanner_2)
                    {
                        if reference_scanner.is_none() {
                            reference_scanner = Some(i);
                            scanner_abs_pos_rot.insert(i, (Vec3::zero(), Mat3::identity()));
                            beacon_list.extend(scanner_1);
                        }
                        if !scanner_abs_pos_rot.contains_key(&i) {
                            if let Some((abs_pos_2, abs_rot_2)) = scanner_abs_pos_rot.get(&j) {
                                let abs_rot_1 = *abs_rot_2 * rot_2_to_1;
                                let abs_beac_pos =
                                    scanner_2[beac_2] * abs_rot_2.transposed() + abs_pos_2;
                                let abs_pos_1 =
                                    abs_beac_pos - (scanner_1[beac_1] * abs_rot_1.transposed());
                                scanner_abs_pos_rot.insert(i, (abs_pos_1, abs_rot_1));
                                beacon_list.extend(
                                    scanner_1
                                        .iter()
                                        .map(|pos| *pos * abs_rot_1.transposed() + abs_pos_1),
                                );
                                // let test = scanner_1.iter().map(|pos| *pos * abs_rot_1.transposed() + abs_pos_1).collect::<Vec<_>>();
                                // dbg!(i);
                                // dbg!(test);
                            }
                        }
                        if !scanner_abs_pos_rot.contains_key(&j) {
                            if let Some((abs_pos_1, abs_rot_1)) = scanner_abs_pos_rot.get(&i) {
                                let abs_rot_2 = *abs_rot_1 * rot_2_to_1.transposed();
                                let abs_beac_pos =
                                    scanner_1[beac_1] * abs_rot_1.transposed() + abs_pos_1;
                                let abs_pos_2 =
                                    abs_beac_pos - (scanner_2[beac_2] * abs_rot_2.transposed());
                                scanner_abs_pos_rot.insert(j, (abs_pos_2, abs_rot_2));
                                beacon_list.extend(
                                    scanner_2
                                        .iter()
                                        .map(|pos| *pos * abs_rot_2.transposed() + abs_pos_2),
                                );
                                // let test = scanner_2.iter().map(|pos| *pos * abs_rot_2.transposed() + abs_pos_2).collect::<Vec<_>>();
                                // dbg!(j);
                                // dbg!(test);
                            }
                        }
                    }
                }
            }
        }
    }
    for _ in 0..5 {
        dbg!(scanner_abs_pos_rot.len());
        dbg!(beacon_list.len());
        for i in 1..input.len() {
            if !scanner_abs_pos_rot.contains_key(&i) {
                let scanner_2 = &input[i];
                let scanner_1 = beacon_list.iter().copied().collect::<Vec<_>>();
                if let Some((beac_1, rot_2_to_1, beac_2)) =
                    overlapping_beacons(&scanner_1, scanner_2)
                {
                    if !scanner_abs_pos_rot.contains_key(&i) {
                        if let Some((abs_pos_1, abs_rot_1)) = scanner_abs_pos_rot.get(&0) {
                            let abs_rot_2 = *abs_rot_1 * rot_2_to_1.transposed();
                            let abs_beac_pos =
                                scanner_1[beac_1] * abs_rot_1.transposed() + abs_pos_1;
                            let abs_pos_2 =
                                abs_beac_pos - (scanner_2[beac_2] * abs_rot_2.transposed());
                            scanner_abs_pos_rot.insert(i, (abs_pos_2, abs_rot_2));
                            beacon_list.extend(
                                scanner_2
                                    .iter()
                                    .map(|pos| *pos * abs_rot_2.transposed() + abs_pos_2),
                            );
                            // let test = scanner_2.iter().map(|pos| *pos * abs_rot_2.transposed() + abs_pos_2).collect::<Vec<_>>();
                            // dbg!(j);
                            // dbg!(test);
                        }
                    }
                }
            }
        }
    }
    // dbg!(scanner_abs_pos_rot);
    dbg!(beacon_list.len());
}

fn overlapping_beacons(
    scanner_1: &[Vec3<i32>],
    scanner_2: &[Vec3<i32>],
) -> Option<(usize, Mat3<i32>, usize)> {
    for (i, beacon_1) in scanner_1.iter().enumerate() {
        let rel_b1_locs = scanner_1
            .iter()
            .map(|pos| pos - beacon_1)
            .collect::<Vec<_>>();
        for rot in rot_mats() {
            let rot_b2_locs = scanner_2.iter().map(|pos| *pos * rot).collect::<Vec<_>>();
            for (j, (beacon_2, _test)) in rot_b2_locs.iter().zip(scanner_2.iter()).enumerate() {
                if rot_b2_locs
                    .iter()
                    .map(|pos| *pos - beacon_2)
                    .filter(|pos| rel_b1_locs.contains(pos))
                    .count()
                    >= 12
                {
                    // dbg!(rot);
                    // dbg!(beacon_1);
                    // dbg!(beacon_2);
                    // dbg!(test);
                    // dbg!(beacon_1 - beacon_2);
                    return Some((i, rot, j));
                }
            }
        }
    }
    None
}

#[derive(Clone, Deserialize)]
struct Input(Vec<Vec<Vec3<i32>>>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}

fn rot_x() -> Mat3<i32> {
    // Mat3::new(1, 0, 0, 0, 0, -1, 0, 1, 0)
    Mat3::new(1, 0, 0, 0, 1, 0, 0, -1, 0)
}

fn rot_y() -> Mat3<i32> {
    // Mat3::new(0, 0, 1, 0, 1, 0, -1, 0, 0)
    Mat3::new(0, 0, -1, 0, 1, 0, 1, 0, 0)
}

fn rot_z() -> Mat3<i32> {
    // Mat3::new(0, -1, 0, 1, 0, 0, 0, 0, 1)
    Mat3::new(0, 1, 0, -1, 0, 0, 0, 0, 1)
}

fn rot_mats() -> [Mat3<i32>; 24] {
    [
        Mat3::identity(),
        rot_x(),
        rot_x() * rot_x(),
        rot_x() * rot_x() * rot_x(),
        rot_y(),
        rot_y() * rot_z(),
        rot_y() * rot_z() * rot_z(),
        rot_y() * rot_z() * rot_z() * rot_z(),
        rot_y() * rot_y(),
        rot_y() * rot_y() * rot_x(),
        rot_y() * rot_y() * rot_x() * rot_x(),
        rot_y() * rot_y() * rot_x() * rot_x() * rot_x(),
        rot_y() * rot_y() * rot_y(),
        rot_y() * rot_y() * rot_y() * rot_z(),
        rot_y() * rot_y() * rot_y() * rot_z() * rot_z(),
        rot_y() * rot_y() * rot_y() * rot_z() * rot_z() * rot_z(),
        rot_z(),
        rot_z() * rot_y(),
        rot_z() * rot_y() * rot_y(),
        rot_z() * rot_y() * rot_y() * rot_y(),
        rot_z() * rot_z() * rot_z(),
        rot_z() * rot_z() * rot_z() * rot_y(),
        rot_z() * rot_z() * rot_z() * rot_y() * rot_y(),
        rot_z() * rot_z() * rot_z() * rot_y() * rot_y() * rot_y(),
    ]
}
