pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
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
    let day12 = day12::AocDay12::new();
    println!("Day 12 - Part 1: {}", day12.part1());
    println!("Day 12 - Part 2: {}", day12.part2());
}
