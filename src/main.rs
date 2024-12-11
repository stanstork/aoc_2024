pub mod day1;
pub mod day2;
pub mod utils;

fn main() {
    let day2 = day2::AocDay2::new(utils::read_lines("data/day2/input.txt"));
    println!("Day 2: {}", day2.day1());
    println!("Day 2: {}", day2.day2());
}
