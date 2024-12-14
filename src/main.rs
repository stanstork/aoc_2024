use day5::AocDay5;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

pub mod utils;

fn main() {
    let input = utils::read_lines("data/day5.txt");
    let day5 = AocDay5::new(input);

    println!("Part 1: {}", day5.part1());
    println!("Part 2: {}", day5.part2());
}
