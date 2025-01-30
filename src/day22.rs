use crate::{utils::read_lines, AocDay};
use std::collections::HashMap;

const MODULO: isize = 16777216;
const MULTIPLIER_1: isize = 64;
const MULTIPLIER_2: isize = 2048;
const DIVISOR: isize = 32;

pub struct AocDay22 {
    nums: Vec<isize>,
}

impl AocDay22 {
    pub fn new() -> Self {
        AocDay22 {
            nums: read_lines("input/day22.txt")
                .iter()
                .map(|line| line.parse::<isize>().unwrap())
                .collect(),
        }
    }

    pub fn part1(&self) -> isize {
        let mut memo = HashMap::new();
        self.nums
            .iter()
            .map(|&num| Self::last_secret(num, &mut memo))
            .sum()
    }

    pub fn part2(&self) -> isize {
        let mut sequences_map = HashMap::new();

        for num in &self.nums {
            let mut memo = HashMap::new();
            let mut secrets = vec![];

            Self::generate_secrets(*num, 2000, &mut memo, &mut secrets);

            let prices = secrets.iter().map(|&s| s % 10).collect::<Vec<_>>();
            let level = Self::extract_sequences(&prices).into_iter().fold(
                HashMap::new(),
                |mut acc, (pattern, value)| {
                    acc.entry(pattern).or_insert_with(Vec::new).push(value);
                    acc
                },
            );

            for (pattern, values) in level {
                sequences_map
                    .entry(pattern)
                    .or_insert_with(Vec::new)
                    .push(values[0]);
            }
        }

        sequences_map
            .values()
            .map(|s| s.iter().sum::<isize>())
            .max()
            .unwrap()
    }

    fn last_secret(secret: isize, memo: &mut HashMap<(isize, usize), isize>) -> isize {
        let mut secrets = vec![];
        Self::generate_secrets(secret, 2000, memo, &mut secrets);
        *secrets.last().unwrap()
    }

    fn generate_secrets(
        secret: isize,
        depth: usize,
        memo: &mut HashMap<(isize, usize), isize>,
        nums: &mut Vec<isize>,
    ) {
        if depth == 0 {
            return;
        }

        if let Some(&result) = memo.get(&(secret, depth)) {
            nums.push(result);
            return;
        }

        let next = Self::next(secret);
        nums.push(next);

        Self::generate_secrets(next, depth - 1, memo, nums);
        memo.insert((secret, depth), next);
    }

    fn next(mut secret: isize) -> isize {
        secret = secret ^ (secret * MULTIPLIER_1) % MODULO;
        secret = secret ^ (secret / DIVISOR) % MODULO;
        secret = secret ^ (secret * MULTIPLIER_2) % MODULO;
        secret
    }

    fn extract_sequences(digits: &[isize]) -> Vec<((isize, isize, isize, isize), isize)> {
        digits
            .windows(5)
            .map(|w| ((w[1] - w[0], w[2] - w[1], w[3] - w[2], w[4] - w[3]), w[4]))
            .collect()
    }
}

impl AocDay for AocDay22 {
    fn part1(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part1())
    }

    fn part2(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part2())
    }
}
