pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
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
    let day13 = day13::AocDay13::new();
    println!("Day 13, part 1: {}", day13.part1());
    println!("Day 13, part 2: {}", day13.part2());
}
