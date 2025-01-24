pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;

pub mod utils;

fn main() {
    let day23 = day23::AocDay23::new();
    println!("Day 23: Part 1 = {}", day23.part1());
    println!("Day 23: Part 2 = {}", day23.part2());
}
