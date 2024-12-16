pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub mod utils;

fn main() {
    let day6 = day6::AocDay6::new();
    println!("Day 6, part 1: {}", day6.part1());
    println!("Day 6, part 2: {}", day6.part2());
}
