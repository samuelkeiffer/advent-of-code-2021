use std::ops::RangeInclusive;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let mut in_range: u32 = 0;
    for y in -1000..1000 {
        for x in 0..1000 {
            if test_velocity(RANGE_X, RANGE_Y, Vec2 { x, y }) {
                in_range += 1;
            }
        }
    }
    dbg!(in_range);
}

fn part1() {
    let mut highest_y = i32::MIN;
    for y in -1000..1000 {
        for x in 0..1000 {
            if let Some(y) = max_y_in_range(RANGE_X, RANGE_Y, Vec2 { x, y }) {
                highest_y = highest_y.max(y);
            }
        }
    }
    dbg!(highest_y);
}

fn test_velocity(
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    mut vel: Vec2,
) -> bool {
    let mut pos = Vec2 { x: 0, y: 0 };
    for _ in 0..1000 {
        if x_range.contains(&pos.x) && y_range.contains(&pos.y) {
            return true;
        }
        step(&mut vel, &mut pos);
    }
    false
}

fn max_y_in_range(
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    mut vel: Vec2,
) -> Option<i32> {
    let mut pos = Vec2 { x: 0, y: 0 };
    let mut highest_y = i32::MIN;
    for _ in 0..1000 {
        highest_y = highest_y.max(pos.y);
        if x_range.contains(&pos.x) && y_range.contains(&pos.y) {
            return Some(highest_y);
        }
        step(&mut vel, &mut pos);
    }
    None
}

fn step(vel: &mut Vec2, pos: &mut Vec2) {
    pos.x += vel.x;
    pos.y += vel.y;
    match vel.x {
        x if x > 0 => vel.x -= 1,
        x if x < 0 => vel.x += 1,
        _ => {}
    }
    vel.y -= 1;
}

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

const RANGE_X: RangeInclusive<i32> = 70..=125;
const RANGE_Y: RangeInclusive<i32> = -159..=-121;
// const TEST_RANGE_X: RangeInclusive<i32> = 20..=30;
// const TEST_RANGE_Y: RangeInclusive<i32> = -10..=-5;
