use std::{collections::VecDeque, thread, time::Duration};

const DEBUG: bool = false;
const DEBUG_SLEEP_MILLIS: u64 = 1000;
const TEST_INPUT: InputKind = InputKind::Actual;

#[allow(dead_code)]
enum InputKind {
    Test1,
    Test2,
    Actual,
}

#[derive(Debug, PartialEq, Eq)]
enum Motion {
    Up,
    Down,
    Left,
    Right,
}

pub fn main() {
    let input: Vec<(Motion, usize)> = match TEST_INPUT {
        InputKind::Test1 => {
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        }
        InputKind::Test2 => {
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
        }
        InputKind::Actual => include_str!("../inputs/day9.txt"),
    }
    .lines()
    .map(|line| {
        let (motion, num) = line.split_once(' ').unwrap();
        (
            match motion {
                "U" => Motion::Up,
                "D" => Motion::Down,
                "L" => Motion::Left,
                "R" => Motion::Right,
                _ => panic!("invalid motion `{motion}`"),
            },
            num.parse().unwrap(),
        )
    })
    .collect();
    println!("--- Day 9 ---");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    println!();
}

fn do_motion(
    motion: &Motion,
    positions: &mut [(usize, usize)],
    visited_cells: &mut VecDeque<VecDeque<bool>>,
) {
    match motion {
        Motion::Up => {
            if visited_cells.get(positions[0].1.wrapping_sub(1)).is_none() {
                for pos in positions.iter_mut() {
                    pos.1 += 1;
                }
                visited_cells.push_front(VecDeque::from(vec![false; visited_cells[0].len()]));
            }
            positions[0].1 -= 1;
        }
        Motion::Down => {
            if visited_cells.get(positions[0].1 + 1).is_none() {
                visited_cells.push_back(VecDeque::from(vec![false; visited_cells[0].len()]));
            }
            positions[0].1 += 1;
        }
        Motion::Left => {
            if visited_cells[positions[0].1]
                .get(positions[0].0.wrapping_sub(1))
                .is_none()
            {
                for pos in positions.iter_mut() {
                    pos.0 += 1;
                }
                for row in visited_cells {
                    row.push_front(false);
                }
            }
            positions[0].0 -= 1;
        }
        Motion::Right => {
            if visited_cells[positions[0].1]
                .get(positions[0].0 + 1)
                .is_none()
            {
                for row in visited_cells {
                    row.push_back(false);
                }
            }
            positions[0].0 += 1;
        }
    }
}

fn part1(input: &[(Motion, usize)]) -> usize {
    let mut visited_cells = VecDeque::from([VecDeque::from([true])]);
    let mut positions = [(0, 0), (0, 0)];
    for (motion, count) in input {
        for _ in 0..*count {
            if (*motion == Motion::Up && positions[1].1 > positions[0].1)
                || (*motion == Motion::Down && positions[1].1 < positions[0].1)
                || (*motion == Motion::Left && positions[1].0 > positions[0].0)
                || (*motion == Motion::Right && positions[1].0 < positions[0].0)
            {
                positions[1] = positions[0]
            }
            do_motion(motion, &mut positions, &mut visited_cells);
            visited_cells[positions[1].1][positions[1].0] = true;
        }
    }
    visited_cells
        .into_iter()
        .flatten()
        .filter(|cell| *cell)
        .count()
}

fn part2(input: &[(Motion, usize)]) -> usize {
    let mut visited_cells = VecDeque::from([VecDeque::from([true])]);
    let mut positions: [(usize, usize); 10] = [(0, 0); 10];
    if DEBUG {
        // clear screen and hide cursor
        print!("\x1b[2J\x1b[?25l");
    }
    for (motion, count) in input {
        for _ in 0..*count {
            do_motion(motion, &mut positions, &mut visited_cells);
            for idx in 0..positions.len() - 1 {
                let (moved, to_move) = (positions[idx], &mut positions[idx + 1]);

                let up = to_move.1 > moved.1 + 1;
                let down = to_move.1 + 1 < moved.1;
                let left = to_move.0 > moved.0 + 1;
                let right = to_move.0 + 1 < moved.0;
                let smaller_x = to_move.0 < moved.0;
                let smaller_y = to_move.1 < moved.1;
                let bigger_x = to_move.0 > moved.0;
                let bigger_y = to_move.1 > moved.1;

                if right || (smaller_x && (up || down)) {
                    to_move.0 += 1;
                } else if left || (bigger_x && (up || down)) {
                    to_move.0 -= 1;
                }
                if down || (smaller_y && (left || right)) {
                    to_move.1 += 1;
                } else if up || (bigger_y && (left || right)) {
                    to_move.1 -= 1;
                }
            }
            let (tail_x, tail_y) = positions.last().unwrap();
            visited_cells[*tail_y][*tail_x] = true;

            if DEBUG {
                // go to top left corner
                print!("\x1b[H");
                const X_SIZE: usize = 60;
                const Y_SIZE: usize = 15;
                let (x, y) = positions[0];
                let x_len = visited_cells[0].len();
                let y_len = visited_cells.len();
                let x_range = match x_len <= X_SIZE * 2 {
                    true => 0..x_len,
                    false => match x {
                        0..=X_SIZE => 0..x_len.min(X_SIZE * 2),
                        _ => (x - X_SIZE)..x_len.min(x + X_SIZE),
                    },
                };
                let y_range = match y_len <= Y_SIZE * 2 {
                    true => 0..y_len,
                    false => match y {
                        0..=Y_SIZE => 0..y_len.min(Y_SIZE * 2),
                        _ => (y - Y_SIZE)..y_len.min(y + Y_SIZE),
                    },
                };

                let border = "-".repeat(x_range.len() + 2);
                // print upper border and clear rest of line
                println!("{border}\x1b[K");
                for y in y_range {
                    // left border
                    print!("|");
                    for x in x_range.clone() {
                        match positions.iter().position(|pos| *pos == (x, y)) {
                            // bold green `H` for head
                            Some(0) => print!("\x1b[1;32mH\x1b[0m"),
                            // bold white number
                            Some(idx) => print!("\x1b[1m{idx}\x1b[0m"),
                            None => match visited_cells[y][x] {
                                // light gray `#` for visited cells
                                true => print!("\x1b[90m#\x1b[0m"),
                                // dark gray `.` for everything else
                                false => print!("\x1b[30m.\x1b[0m"),
                            },
                        }
                    }
                    // right border and clear rest of line
                    println!("|\x1b[K");
                }
                // bottom border and clear rest of screen
                println!("{border}\x1b[J");
                thread::sleep(Duration::from_millis(DEBUG_SLEEP_MILLIS));
            }
        }
    }
    if DEBUG {
        print!("\x1b[?25h");
    }
    visited_cells
        .into_iter()
        .flatten()
        .filter(|cell| *cell)
        .count()
}
