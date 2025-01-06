pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day15_2;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
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
    let day19 = day19::AocDay19::new();
    println!("Part 1: {}", day19.part1());
    println!("Part 2: {}", day19.part2());
}
