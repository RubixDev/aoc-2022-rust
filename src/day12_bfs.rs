use std::collections::VecDeque;

use strum::IntoEnumIterator;

use crate::day12::{offset, parse_heightmap, part1, part2, Direction};

pub fn main() {
    let (start_pos, end_pos, heightmap) = parse_heightmap();

    let mut step_counts = vec![vec![None; heightmap[0].len()]; heightmap.len()];
    step_counts[end_pos.1][end_pos.0] = Some(0);

    let mut queue = VecDeque::from([(0, end_pos)]);
    while let Some((d, (x, y))) = queue.pop_front() {
        for direction in Direction::iter() {
            let (ox, oy) = offset((x, y), direction);
            if !(0..step_counts[0].len()).contains(&ox)
                || !(0..step_counts.len()).contains(&oy)
                || heightmap[y][x] > heightmap[oy][ox].saturating_add(1)
                || step_counts[oy][ox].is_some()
            {
                continue;
            }
            step_counts[oy][ox] = Some(d + 1);
            if (ox, oy) == start_pos {
                break;
            }
            queue.push_back((d + 1, (ox, oy)));
        }
    }

    println!("--- Day 12 (BFS) ---");
    println!("Part 1: {}", part1(&step_counts, start_pos));
    println!("Part 2: {}", part2(&step_counts, &heightmap));
}
