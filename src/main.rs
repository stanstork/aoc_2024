pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub mod utils;

fn main() {
    let input = utils::read_lines("data/day7.txt");
    let aoc = day7::AocDay7::new(input);

    println!("Day 7, Part 1: {}", aoc.part1());
    println!("Day 7, Part 2: {}", aoc.part2());
}
