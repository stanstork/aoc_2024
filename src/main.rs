pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day15_2;
pub mod day2;
pub mod day24;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub mod utils;

fn main() {
    let day15 = day15::AocDay15::new();
    println!("Day 15, part 1: {}", day15.part1());
    println!("Day 15, part 2: {}", day15.part2());
}
