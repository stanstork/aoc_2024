use crate::utils::{in_bounds, read_matrix};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    vec,
};

pub struct AocDay20 {
    matrix: Vec<Vec<char>>,
    start: (isize, isize),
    end: (isize, isize),
}

impl AocDay20 {
    pub fn new() -> AocDay20 {
        let matrix = read_matrix("data/day20.txt");
        let (start, end) = Self::find_start_end(&matrix);

        AocDay20 { matrix, start, end }
    }

    pub fn part1(&self) -> usize {
        self.calc_saved_steps(2, 100)
    }

    pub fn part2(&self) -> usize {
        self.calc_saved_steps(20, 100)
    }

    fn calc_saved_steps(&self, max_cheats: isize, min_saved: isize) -> usize {
        let path = self.get_path();
        let mut freq = HashMap::new();

        for (i, &current) in path.iter().enumerate() {
            for (j, &next) in path.iter().enumerate() {
                if i == j {
                    continue;
                }

                let direct_dist = Self::manhattan((current.0, current.1), (next.0, next.1));
                let path_dist = i - j;

                if direct_dist <= max_cheats {
                    let saved = path_dist as isize - direct_dist;
                    *freq.entry(saved).or_insert(0) += 1;
                }
            }
        }

        freq.iter()
            .filter(|(&k, _)| k >= min_saved)
            .map(|(_, &v)| v)
            .sum()
    }

    fn get_path(&self) -> Vec<(isize, isize)> {
        let paths = Self::find_paths(&self.matrix, self.start, self.end);
        let shortest = paths.iter().min_by_key(|p| p.len()).unwrap();
        shortest.clone()
    }

    fn manhattan(a: (isize, isize), b: (isize, isize)) -> isize {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }

    fn find_paths(
        matrix: &Vec<Vec<char>>,
        start: (isize, isize),
        end: (isize, isize),
    ) -> Vec<Vec<(isize, isize)>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut paths = Vec::new();

        queue.push_back(vec![start]);
        visited.insert(start);

        while let Some(path) = queue.pop_front() {
            let (row, col) = path.last().unwrap();

            if !in_bounds(matrix, *row, *col) || matrix[*row as usize][*col as usize] == '#' {
                continue;
            }

            if *row == end.0 && *col == end.1 {
                paths.push(path.clone());
                continue;
            }

            for &(row_offset, col_offset) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let new_row = row + row_offset;
                let new_col = col + col_offset;

                if visited.contains(&(new_row, new_col)) {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push((new_row, new_col));

                queue.push_back(new_path);
                visited.insert((new_row, new_col));
            }
        }

        paths
    }

    fn find_start_end(matrix: &Vec<Vec<char>>) -> ((isize, isize), (isize, isize)) {
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (i, row) in matrix.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == 'S' {
                    start = (i, j);
                } else if *c == 'E' {
                    end = (i, j);
                }
            }
        }

        (
            (start.0 as isize, start.1 as isize),
            (end.0 as isize, end.1 as isize),
        )
    }
}
