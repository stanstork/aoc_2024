use day4::AocDay4;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod utils;

fn main() {
    let mut day4 = AocDay4::new(utils::read_lines("data/day4/input.txt"));
    println!("Part 1: {}", day4.part1());
    println!("Part 2: {}", day4.part2());
}
