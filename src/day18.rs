use crate::utils::{self, in_bounds};
use std::collections::{HashSet, VecDeque};

pub struct AocDay18 {
    lines: Vec<String>,
    matrix: Vec<Vec<char>>,
}

impl AocDay18 {
    pub fn new() -> AocDay18 {
        let lines = utils::read_lines("input/day18.txt");
        let mut matrix = vec![vec!['.'; 71]; 71];

        for i in 0..1024 {
            let line = &lines[i];
            let pos = Self::get_position(line);

            matrix[pos.1][pos.0] = '#';
        }

        AocDay18 { lines, matrix }
    }

    pub fn part1(&self) -> isize {
        let matrix = self.matrix.clone();
        let paths = Self::find_paths(&matrix);
        let shortest = paths.iter().min_by_key(|p| p.len()).unwrap();

        shortest.len() as isize - 1
    }

    pub fn part2(&self) -> String {
        let mut matrix = self.matrix.clone();

        for i in 1024..self.lines.len() {
            let line = &self.lines[i];
            let pos = Self::get_position(line);

            matrix[pos.1][pos.0] = '#';

            let paths = Self::find_paths(&matrix);
            if paths.len() == 0 {
                return format!("{}, {}", pos.0, pos.1);
            }
        }

        panic!("No solution found");
    }

    fn find_paths(matrix: &Vec<Vec<char>>) -> Vec<Vec<(isize, isize)>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut paths = Vec::new();

        queue.push_back(vec![(0, 0)]);
        visited.insert((0, 0));

        while let Some(path) = queue.pop_front() {
            let (row, col) = path.last().unwrap();

            if !in_bounds(matrix, *row, *col) || matrix[*row as usize][*col as usize] == '#' {
                continue;
            }

            if *row == (matrix.len() - 1) as isize && *col == (matrix[0].len() - 1) as isize {
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

    fn get_position(line: &String) -> (usize, usize) {
        let pos = line
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        (pos[0], pos[1])
    }
}
