use crate::utils;
use std::{collections::HashMap, vec};

pub struct AocDay11 {
    nums: Vec<i64>,
}

impl AocDay11 {
    pub fn new() -> Self {
        AocDay11 {
            nums: Self::get_nums(),
        }
    }

    pub fn part1(&self) -> usize {
        self.calc(25)
    }

    pub fn part2(&self) -> usize {
        self.calc(75)
    }

    fn calc(&self, blinks: usize) -> usize {
        let mut memo = &mut HashMap::new();
        self.nums
            .iter()
            .map(|&n| Self::split(n, 0, blinks, &mut memo))
            .sum()
    }

    fn split(
        n: i64,
        depth: usize,
        max_depth: usize,
        memo: &mut HashMap<(i64, usize), usize>,
    ) -> usize {
        if let Some(&result) = memo.get(&(n, depth)) {
            return result;
        }

        if depth >= max_depth {
            return 1;
        }

        let parts = if n == 0 {
            vec![1]
        } else {
            let num_digits = Self::num_digits(n);
            match num_digits % 2 {
                0 => {
                    let (left, right) = Self::split_digits(n, num_digits);
                    vec![left, right]
                }
                _ => vec![n * 2024],
            }
        };

        let sum = parts
            .iter()
            .map(|&part| Self::split(part, depth + 1, max_depth, memo))
            .sum();
        memo.insert((n, depth), sum);

        sum
    }

    fn split_digits(n: i64, digits: usize) -> (i64, i64) {
        let divisor = 10_i64.pow((digits / 2) as u32);
        (n / divisor, n % divisor)
    }

    fn num_digits(n: i64) -> usize {
        if n == 0 {
            1
        } else {
            (n as f64).log10().floor() as usize + 1
        }
    }

    fn get_nums() -> Vec<i64> {
        let input = utils::read_lines("input/day11.txt")[0].clone();
        input
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }
}
