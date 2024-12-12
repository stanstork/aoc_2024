use day3::AocDay3;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod utils;

fn main() {
    let mut aoc_day3 = AocDay3::new(utils::read_lines("data/day3/input.txt"));
    println!("Part 1: {}", aoc_day3.part1());
    println!("Part 2: {}", aoc_day3.part2());
}
