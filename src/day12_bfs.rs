use std::{collections::VecDeque, thread, time::Duration};

use strum::IntoEnumIterator;

use crate::day12::{offset, parse_heightmap, part1, part2, Direction, Pos};

const TEST: bool = false;
const DEBUG: bool = false;
const DEBUG_SLEEP_MILLIS: u64 = 0;

pub fn main() {
    let (start_pos, end_pos, heightmap) = parse_heightmap(TEST);

    let mut step_counts = vec![vec![None; heightmap[0].len()]; heightmap.len()];
    step_counts[end_pos.1][end_pos.0] = Some(0);

    if DEBUG {
        print!("\x1b[2J\x1b[?25l");
    }
    let mut queue = VecDeque::from([(vec![end_pos], end_pos)]);
    while let Some((path, (x, y))) = queue.pop_front() {
        for direction in Direction::iter() {
            let (ox, oy) = offset((x, y), direction);
            if !(0..step_counts[0].len()).contains(&ox)
                || !(0..step_counts.len()).contains(&oy)
                || heightmap[y][x] > heightmap[oy][ox].saturating_add(1)
                || step_counts[oy][ox].is_some()
            {
                continue;
            }
            step_counts[oy][ox] = Some(path.len());

            if DEBUG {
                print_map((ox, oy), &step_counts, &path);
            }

            if (ox, oy) == start_pos {
                break;
            }
            let mut path = path.clone();
            path.push((ox, oy));
            queue.push_back((path, (ox, oy)));
        }
    }
    if DEBUG {
        print!("\x1b[?25h");
    }

    println!("--- Day 12 (BFS) ---");
    println!("Part 1: {}", part1(&step_counts, start_pos));
    println!("Part 2: {}", part2(&step_counts, &heightmap));
}

fn print_map(pos: Pos, map: &[Vec<Option<usize>>], path: &[Pos]) {
    print!("\x1b[H");
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if (x, y) == pos {
                print!("\x1b[42m");
            } else if path.contains(&(x, y)) {
                print!("\x1b[46m");
            }
            match cell {
                Some(_) => print!("X"),
                None => print!("\x1b[90m×"),
            }
            print!("\x1b[0m");
        }
        println!();
    }
    thread::sleep(Duration::from_millis(DEBUG_SLEEP_MILLIS));
}
