use crate::{
    utils::{in_bounds, read_matrix},
    AocDay,
};
use std::collections::{HashMap, HashSet};

pub struct AocDay12 {
    plot: Vec<Vec<char>>,
}

impl AocDay12 {
    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    pub fn new() -> Self {
        AocDay12 {
            plot: read_matrix("input/day12.txt"),
        }
    }

    pub fn part1(&self) -> usize {
        self.calc_price(|visited, _, perimeter| visited.len() * perimeter)
    }

    pub fn part2(&self) -> usize {
        let plot = &self.plot;
        self.calc_price(|visited, target, _| {
            let walls = Self::get_walls_from_region(visited, plot, *target);
            let sides = walls
                .iter()
                .flat_map(|wall| wall.values())
                .map(Self::count_distinct_walls)
                .sum::<usize>();
            visited.len() * sides
        })
    }

    fn calc_price<F>(&self, price_fn: F) -> usize
    where
        F: Fn(&HashSet<(isize, isize)>, &char, usize) -> usize,
    {
        let mut plot = self.plot.clone();
        let mut total_price = 0;

        for r in 0..plot.len() {
            for c in 0..plot[0].len() {
                if plot[r][c] != '.' {
                    let target = plot[r][c];

                    let mut visited = HashSet::new();
                    let mut perimeter = 0;
                    Self::dfs(
                        &mut plot,
                        r as isize,
                        c as isize,
                        target,
                        &mut visited,
                        &mut perimeter,
                    );

                    total_price += price_fn(&visited, &target, perimeter);
                }
            }
        }

        total_price
    }

    fn dfs(
        matrix: &mut Vec<Vec<char>>,
        r: isize,
        c: isize,
        target: char,
        region: &mut HashSet<(isize, isize)>,
        perimeter: &mut usize,
    ) {
        if !in_bounds(matrix, r, c) || matrix[r as usize][c as usize] != target {
            return;
        }

        region.insert((r, c));
        matrix[r as usize][c as usize] = '.';

        for (dr, dc) in Self::DIRECTIONS.iter() {
            let new_r = r + dr;
            let new_c = c + dc;

            if !region.contains(&(new_r, new_c)) {
                if !in_bounds(matrix, new_r, new_c)
                    || matrix[new_r as usize][new_c as usize] != target
                {
                    *perimeter += 1;
                }
                Self::dfs(matrix, new_r, new_c, target, region, perimeter);
            }
        }
    }

    fn get_walls_from_region(
        region: &HashSet<(isize, isize)>,
        matrix: &Vec<Vec<char>>,
        target: char,
    ) -> Vec<HashMap<isize, Vec<isize>>> {
        let mut left_sides = HashMap::new();
        let mut right_sides = HashMap::new();
        let mut top_sides = HashMap::new();
        let mut bottom_sides = HashMap::new();

        for (r, c) in region.iter() {
            for (dr, dc) in Self::DIRECTIONS.iter() {
                let new_r = r + dr;
                let new_c = c + dc;

                let is_wall = |r, c| {
                    if in_bounds(matrix, r, c) {
                        matrix[r as usize][c as usize] != target
                    } else {
                        true // Out of bounds is considered a wall
                    }
                };

                if *dc == -1 && is_wall(new_r, new_c) {
                    left_sides.entry(*c).or_insert(Vec::new()).push(*r);
                } else if *dc == 1 && is_wall(new_r, new_c) {
                    right_sides.entry(*c).or_insert(Vec::new()).push(*r);
                } else if *dr == -1 && is_wall(new_r, new_c) {
                    top_sides.entry(*r).or_insert(Vec::new()).push(*c);
                } else if *dr == 1 && is_wall(new_r, new_c) {
                    bottom_sides.entry(*r).or_insert(Vec::new()).push(*c);
                }
            }
        }

        vec![left_sides, right_sides, top_sides, bottom_sides]
    }

    fn count_distinct_walls(indices: &Vec<isize>) -> usize {
        let mut indices = indices.clone();
        indices.sort();

        if indices.is_empty() {
            return 0;
        }

        let mut count = 1;
        let mut prev_index = indices[0];

        for &index in indices.iter().skip(1) {
            if index - prev_index > 1 {
                count += 1;
            }
            prev_index = index;
        }

        count
    }
}

impl AocDay for AocDay12 {
    fn part1(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part1())
    }

    fn part2(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part2())
    }
}
