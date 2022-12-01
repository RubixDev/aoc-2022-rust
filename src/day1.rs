pub fn main() {
    let input = include_str!("../inputs/day1.txt")
        .trim()
        .split("\n\n")
        .map(|inv| {
            inv.split('\n')
                .map(|cnt| cnt.parse::<u64>().unwrap())
                .collect()
        })
        .collect::<Vec<_>>();
    println!("--- Day 1 ---");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    println!();
}

fn part1(input: &[Vec<u64>]) -> u64 {
    input.iter().map(|inv| inv.iter().sum()).max().unwrap()
}

fn part2(input: &[Vec<u64>]) -> u64 {
    let mut sums: Vec<_> = input.iter().map(|inv| inv.iter().sum()).collect();
    sums.sort_unstable();
    sums.iter().rev().take(3).sum()
}
