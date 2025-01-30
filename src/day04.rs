use crate::{utils::read_lines, AocDay};

const SEARCH_STRING: [char; 4] = ['X', 'M', 'A', 'S'];

#[derive(Clone)]
pub struct AocDay4 {
    matrix: Vec<Vec<char>>,
}

impl AocDay4 {
    pub fn new() -> AocDay4 {
        let input = read_lines("input/day4.txt");
        let matrix = Self::create_matrix(input);
        AocDay4 { matrix }
    }

    pub fn part1(&self) -> i32 {
        let mut _self = self.clone();
        let mut words = 0;

        let directions = [
            (0, 1),   // right
            (1, 0),   // down
            (1, 1),   // down right
            (1, -1),  // down left
            (0, -1),  // left
            (-1, 0),  // up
            (-1, 1),  // up right
            (-1, -1), // up left
        ];

        for r in 0..self.matrix.len() {
            for c in 0..self.matrix[0].len() {
                if self.matrix[r][c] == SEARCH_STRING[0] {
                    for (dr, dc) in directions.iter() {
                        if Self::search_in_direction(
                            &mut _self.matrix,
                            r as isize,
                            c as isize,
                            *dr,
                            *dc,
                            0,
                        ) {
                            words += 1;
                        }
                    }
                }
            }
        }

        words
    }

    pub fn part2(&self) -> i32 {
        let mut words = 0;

        let diagonals = [
            (1, 1, -1, -1), // down-right and up-left
            (1, -1, -1, 1), // down-left and up-right
        ];

        for r in 0..self.matrix.len() {
            for c in 0..self.matrix[0].len() {
                if self.matrix[r][c] == 'A' {
                    let mut valid_diagonals = 0;

                    for &(dr1, dc1, dr2, dc2) in &diagonals {
                        if Self::check_diagonal(&self.matrix, r, c, dr1, dc1, dr2, dc2) {
                            valid_diagonals += 1;
                        }
                    }

                    if valid_diagonals == 2 {
                        words += 1;
                    }
                }
            }
        }

        words
    }

    fn create_matrix(input: Vec<String>) -> Vec<Vec<char>> {
        let mut matrix = vec![];
        for line in input {
            let row = line.chars().collect::<Vec<_>>();
            matrix.push(row);
        }
        matrix
    }

    fn search_in_direction(
        matrix: &Vec<Vec<char>>,
        r: isize,
        c: isize,
        dr: isize,
        dc: isize,
        i: usize,
    ) -> bool {
        if i == SEARCH_STRING.len() {
            return true;
        }

        if !Self::is_in_bounds(matrix, r, c) {
            return false;
        }

        let (ur, uc) = (r as usize, c as usize);

        if matrix[ur][uc] != SEARCH_STRING[i] {
            return false;
        }

        Self::search_in_direction(matrix, r + dr, c + dc, dr, dc, i + 1)
    }

    fn is_in_bounds(matrix: &[Vec<char>], r: isize, c: isize) -> bool {
        r >= 0 && c >= 0 && (r as usize) < matrix.len() && (c as usize) < matrix[0].len()
    }

    fn check_diagonal(
        matrix: &[Vec<char>],
        r: usize,
        c: usize,
        dr1: isize,
        dc1: isize,
        dr2: isize,
        dc2: isize,
    ) -> bool {
        let r1 = r as isize + dr1;
        let c1 = c as isize + dc1;
        let r2 = r as isize + dr2;
        let c2 = c as isize + dc2;

        Self::is_in_bounds(matrix, r1, c1)
            && Self::is_in_bounds(matrix, r2, c2)
            && ((matrix[r1 as usize][c1 as usize] == 'S'
                && matrix[r2 as usize][c2 as usize] == 'M')
                || (matrix[r1 as usize][c1 as usize] == 'M'
                    && matrix[r2 as usize][c2 as usize] == 'S'))
    }
}

impl AocDay for AocDay4 {
    fn part1(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part1())
    }

    fn part2(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part2())
    }
}
