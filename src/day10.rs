use std::collections::HashSet;

use crate::utils;

pub struct AocDay10 {
    map: Vec<Vec<i32>>,
}

impl AocDay10 {
    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    pub fn new() -> Self {
        AocDay10 {
            map: utils::read_num_matrix("data/day10.txt"),
        }
    }

    pub fn part1(&self) -> usize {
        let mut total = 0;
        for (r, row) in self.map.iter().enumerate() {
            for (c, &value) in row.iter().enumerate() {
                if value == 0 {
                    let mut reached = HashSet::new();
                    self.dfs( r as isize, c as isize, 0, &mut reached);
                    total += reached.len();
                }
            }
        }
        total
    }

    pub fn part2(&self) -> i32 {
        let mut total = 0;
        for (r, row) in self.map.iter().enumerate() {
            for (c, &value) in row.iter().enumerate() {
                if value == 0 {
                    total += self.count_paths(r as isize, c as isize, 0);
                }
            }
        }
        total
    }

    fn dfs(&self, r: isize, c: isize, target: i32, reached: &mut HashSet<(usize, usize)>)  {
        if !self.is_within_bounds(r, c) || self.map[r as usize][c as usize] != target {
            return;
        }
    
        if self.map[r as usize][c as usize] == target && target == 9 {
            reached.insert((r as usize, c as usize));
            return;
        }
    
        for (dr, dc) in Self::DIRECTIONS.iter() {
            self.dfs(r + dr, c + dc, target + 1, reached);
        }
    }

    fn count_paths(&self, r: isize, c: isize, target: i32) -> i32 {
        if !self.is_within_bounds(r, c) || self.map[r as usize][c as usize] != target {
            return 0;
        }
    
        if target == 9 {
            return 1;
        }
    
        Self::DIRECTIONS.iter()
            .map(|&(dr, dc)| self.count_paths(r + dr, c + dc, target + 1))
            .sum()
    }
    

    fn is_within_bounds(&self, r: isize, c: isize) -> bool {
        r >= 0 && (r as usize) < self.map.len() && c >= 0 && (c as usize) < self.map[0].len()
    }
}
