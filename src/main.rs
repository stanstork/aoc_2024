pub mod day1;
pub mod utils;

fn main() {
    let day1 = day1::AocDay1::new(utils::read_lines("data/day1/input.txt"));
    println!("Part 1: {}", day1.part1());
    println!("Part 2: {}", day1.part2());
}
