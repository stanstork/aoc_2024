pub mod day1;
pub mod day10;
pub mod day11;
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
    let day11 = day11::AocDay11::new();
    println!("Day 11 - Part 1: {}", day11.part1());
    println!("Day 11 - Part 2: {}", day11.part2());
}
