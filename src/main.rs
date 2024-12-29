pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day15_2;
pub mod day16;
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
    let day16 = day16::AocDay16::new();
    println!("Day 16, part 1: {}", day16.part1());
    println!("Day 16, part 2: {}", day16.part2());
}
