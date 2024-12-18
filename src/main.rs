pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub mod utils;

fn main() {
    let day9 = day9::AocDay9::new(utils::read_lines("data/day9.txt"));
    println!("Day 9, Part 1: {}", day9.part1());
    println!("Day 9, Part 2: {}", day9.part2());
}
