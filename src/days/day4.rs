use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let numbers: Vec<u32> = get_input::<Input>("day-4").0;
    let boards_in: Vec<u32> = get_input::<Input>("day-4").1;
    let mut boards: Vec<([[(u32, bool); 5]; 5], bool)> = Vec::new();
    for i in 0..(boards_in.len() / 25) {
        let new_line = [(0, false); 5];
        let mut new_array = [new_line; 5];
        for j in 0..5 {
            for k in 0..5 {
                new_array[j][k] = (boards_in[i * 25 + j * 5 + k], false);
            }
        }
        boards.push((new_array, false));
    }
    let mut finish = None;
    for number in numbers {
        // Set number to true
        for (board, _) in boards.iter_mut() {
            for row in board {
                for (entry, ref mut hit) in row {
                    if *entry == number {
                        *hit = true;
                    }
                }
            }
        }
        // Check if any row or column
        let mut boards_done = Vec::new();
        for (k, (board, _)) in boards.iter().enumerate() {
            for i in 0..5 {
                // Row
                if board[i].iter().all(|(_, hit)| *hit) {
                    boards_done.push(k);
                }
                // Column
                if (0..5).into_iter().all(|j| board[j][i].1) {
                    boards_done.push(k);
                }
            }
        }
        for board in boards_done {
            boards[board].1 = true;
        }
        if boards
            .iter()
            .map(|(_, done)| if *done { 0 } else { 1 })
            .sum::<u32>()
            == 1
        {
            let end = boards.iter().enumerate().find_map(
                |(i, (_, done))| {
                    if *done {
                        None
                    } else {
                        Some(i)
                    }
                },
            );
            finish = end;
        }
        if let Some(finish) = finish {
            if boards[finish].1 {
                let mut sum = 0;
                for i in 0..5 {
                    for j in 0..5 {
                        let (num, hit) = boards[finish].0[i][j];
                        if !hit {
                            sum += num;
                        }
                    }
                }
                let prod = sum * number;
                dbg!(prod);
            }
        }
    }
}

fn part1() {
    let numbers: Vec<u32> = get_input::<Input>("day-4").0;
    let boards_in: Vec<u32> = get_input::<Input>("day-4").1;
    let mut boards: Vec<[[(u32, bool); 5]; 5]> = Vec::new();
    for i in 0..(boards_in.len() / 25) {
        let new_line = [(0, false); 5];
        let mut new_array = [new_line; 5];
        for j in 0..5 {
            for k in 0..5 {
                new_array[j][k] = (boards_in[i * 25 + j * 5 + k], false);
            }
        }
        boards.push(new_array);
    }
    for number in numbers {
        // Set number to true
        for board in boards.iter_mut() {
            for row in board {
                for (entry, ref mut hit) in row {
                    if *entry == number {
                        *hit = true;
                    }
                }
            }
        }
        // Check if any row or column
        let mut done = None;
        for (k, board) in boards.iter().enumerate() {
            for i in 0..5 {
                // Row
                if board[i].iter().all(|(_, hit)| *hit) {
                    done = Some(k);
                }
                // Column
                if (0..5).into_iter().all(|j| board[j][i].1) {
                    done = Some(k)
                }
            }
        }
        if let Some(done) = done {
            let mut sum = 0;
            for i in 0..5 {
                for j in 0..5 {
                    let (num, hit) = boards[done][i][j];
                    if !hit {
                        sum += num;
                    }
                }
            }
            let prod = sum * number;
            dbg!(prod);
            break;
        }
    }
}

#[derive(Clone, Deserialize)]
struct Input(Vec<u32>, Vec<u32>);

impl Asset for Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}