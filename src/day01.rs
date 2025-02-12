use crate::{utils::read_lines, AocDay};
use std::{collections::HashMap, fmt::Display};

pub struct AocDay1 {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl AocDay1 {
    pub fn new() -> AocDay1 {
        let input = read_lines("input/day1.txt");
        let (mut left, mut right) = Self::parse(&input);

        left.sort();
        right.sort();

        AocDay1 { left, right }
    }

    pub fn part1(&self) -> i32 {
        self.left
            .iter()
            .zip(self.right.iter())
            .fold(0, |acc, (l, r)| acc + (l - r).abs())
    }

    pub fn part2(&self) -> i32 {
        let right_map = Self::get_map(&self.right);
        let mut score = 0;

        for l in self.left.iter() {
            let times = *right_map.get(&l).unwrap_or(&0);
            score += l * times;
        }

        score
    }

    fn parse(input: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in input {
            let locations = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            left.push(locations[0]);
            right.push(locations[1]);
        }

        (left, right)
    }

    fn get_map(loc: &Vec<i32>) -> HashMap<&i32, i32> {
        loc.iter().fold(HashMap::new(), |mut map, value| {
            map.entry(value).and_modify(|x| *x += 1).or_insert(1);
            map
        })
    }
}

impl AocDay for AocDay1 {
    fn part1(&self) -> Box<dyn Display> {
        Box::new(self.part1())
    }

    fn part2(&self) -> Box<dyn Display> {
        Box::new(self.part2())
    }
}
