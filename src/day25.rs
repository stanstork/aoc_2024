use crate::{utils::split_multi_lines_whitespace, AocDay};

pub struct AocDay25;

impl AocDay25 {
    pub fn new() -> AocDay25 {
        AocDay25
    }

    pub fn part1(&self) -> usize {
        let input = split_multi_lines_whitespace("input/day25.txt");
        let (locks, keys): (Vec<_>, Vec<_>) =
            input.iter().partition(|schema| Self::is_lock(schema));

        let max_height = 5;
        let mut combinations = 0;

        for lock in &locks {
            let lock_pins = Self::get_pin_heights(lock, true);

            for key in &keys {
                let key_pins = Self::get_pin_heights(key, false);

                if lock_pins
                    .iter()
                    .zip(&key_pins)
                    .all(|(&lock, &key)| key <= max_height - lock)
                {
                    combinations += 1;
                }
            }
        }

        combinations
    }

    fn is_lock(schema: &[String]) -> bool {
        schema
            .first()
            .map_or(false, |row| row.chars().all(|c| c == '#'))
    }

    fn get_pin_heights(schema: &[String], is_lock: bool) -> Vec<usize> {
        let width = schema[0].len();
        let height = schema.len();
        let mut heights = vec![0; width];

        for col in 0..width {
            heights[col] = if is_lock {
                (1..height)
                    .take_while(|&row| schema[row].chars().nth(col).unwrap_or('.') == '#')
                    .count()
            } else {
                (0..height - 1)
                    .rev()
                    .take_while(|&row| schema[row].chars().nth(col).unwrap_or('.') == '#')
                    .count()
            };
        }

        heights
    }
}

impl AocDay for AocDay25 {
    fn part1(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part1())
    }

    fn part2(&self) -> Box<dyn std::fmt::Display> {
        Box::new("No part 2")
    }
}
