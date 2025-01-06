use crate::utils::split_lines_whitespace;
use std::collections::{HashMap, HashSet};

pub struct AocDay19 {
    towels: HashSet<String>,
    designs: Vec<String>,
}

impl AocDay19 {
    pub fn new() -> AocDay19 {
        let input = split_lines_whitespace("data/day19.txt");
        let towels = input.0[0]
            .split(',')
            .map(|t| t.trim().to_string())
            .collect::<HashSet<String>>();
        let designs = input.1;

        AocDay19 { towels, designs }
    }

    pub fn part1(&self) -> usize {
        let mut count = 0;
        let mut memo = HashMap::new();

        for design in &self.designs {
            if self.split(&design, &mut memo) {
                count += 1;
            }
        }

        count
    }

    pub fn part2(&self) -> usize {
        let mut total = 0;

        for design in &self.designs {
            let mut memo = HashMap::new();
            let mut count = 0;

            self.split_count(&design, &mut count, &mut memo);

            total += count;
        }

        total
    }

    fn split(&self, design: &str, memo: &mut HashMap<String, bool>) -> bool {
        if design.is_empty() {
            return true;
        }

        if let Some(&valid) = memo.get(design) {
            return valid;
        }

        for i in 1..design.len() + 1 {
            let part = &design[..i];
            if self.towels.contains(part) && self.split(&design[i..], memo) {
                memo.insert(design.to_string(), true);
                return true;
            }
        }

        memo.insert(design.to_string(), false);
        false
    }

    fn split_count(&self, design: &str, count: &mut usize, memo: &mut HashMap<String, usize>) {
        if design.is_empty() {
            *count += 1;
            return;
        }

        if let Some(&cached_count) = memo.get(design) {
            *count += cached_count;
            return;
        }

        let mut current_count = 0;

        for i in 1..=design.len() {
            let part = &design[..i];
            if self.towels.contains(part) {
                self.split_count(&design[i..], &mut current_count, memo);
            }
        }

        memo.insert(design.to_string(), current_count);
        *count += current_count;
    }
}
