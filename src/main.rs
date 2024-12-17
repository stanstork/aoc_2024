pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

pub mod utils;

fn main() {
    let day8 = day8::AocDay8::new(utils::read_matrix("data/day8.txt"));

    println!("Part 1: {}", day8.part1());
    println!("Part 2: {}", day8.part2());
}
