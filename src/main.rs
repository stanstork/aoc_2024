pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;

pub mod utils;

fn main() {
    let day10 = day10::AocDay10::new();
    println!("Day 10, Part 1: {}", day10.part1());
    println!("Day 10, Part 2: {}", day10.part2());
}
