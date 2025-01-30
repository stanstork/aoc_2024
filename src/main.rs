use std::fmt::Display;

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
pub mod day24;
pub mod day25;

pub mod utils;

trait AocDay {
    fn part1(&self) -> Box<dyn Display>;
    fn part2(&self) -> Box<dyn Display>;
}

fn main() {
    let days: Vec<Box<dyn AocDay>> = vec![
        Box::new(day01::AocDay1::new()),
        Box::new(day02::AocDay2::new()),
        Box::new(day03::AocDay3::new()),
        Box::new(day04::AocDay4::new()),
        Box::new(day05::AocDay5::new()),
        Box::new(day06::AocDay6::new()),
        Box::new(day07::AocDay7::new()),
        Box::new(day08::AocDay8::new()),
        Box::new(day09::AocDay9::new()),
        Box::new(day10::AocDay10::new()),
        Box::new(day11::AocDay11::new()),
        Box::new(day12::AocDay12::new()),
        Box::new(day13::AocDay13::new()),
        Box::new(day14::AocDay14::new()),
        Box::new(day15::AocDay15::new()),
        Box::new(day16::AocDay16::new()),
        Box::new(day17::AocDay17::new()),
        Box::new(day18::AocDay18::new()),
        Box::new(day19::AocDay19::new()),
        Box::new(day20::AocDay20::new()),
        Box::new(day21::AocDay21::new()),
        Box::new(day22::AocDay22::new()),
        Box::new(day23::AocDay23::new()),
        Box::new(day24::AocDay24::new()),
        Box::new(day25::AocDay25::new()),
    ];

    for (i, day) in days.iter().enumerate() {
        println!("--- Day {:02} ---", i + 1);
        println!("Part 1: {}", day.part1());
        println!("Part 2: {}", day.part2());
    }
}
