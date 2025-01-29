use crate::utils::{in_bounds, read_matrix};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offsets() -> [(Direction, isize, isize); 4] {
        [
            (Direction::Up, -1, 0),
            (Direction::Down, 1, 0),
            (Direction::Left, 0, -1),
            (Direction::Right, 0, 1),
        ]
    }
}

pub struct AocDay16 {
    pub matrix: Vec<Vec<char>>,
}

impl AocDay16 {
    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    pub fn new() -> AocDay16 {
        let matrix = read_matrix("input/day16.txt");
        AocDay16 { matrix }
    }

    pub fn part1(&self) -> isize {
        let (start, end) = Self::find_start_end(&self.matrix);
        let (_, visited) = Self::find_paths(&self.matrix, start);
        Self::get_min_price(&visited, end)
    }

    pub fn part2(&self) -> usize {
        let matrix = self.matrix.clone();
        let (start, end) = Self::find_start_end(&matrix);
        let (paths, visited) = Self::find_paths(&matrix, start);
        let min_price = Self::get_min_price(&visited, end);

        let mut unique_positions = HashSet::new();
        for path in paths.iter() {
            if path.0 == min_price {
                for (row, col, _) in path.1.iter() {
                    unique_positions.insert((row, col));
                }
            }
        }

        unique_positions.len()
    }

    fn get_min_price(
        visited: &HashMap<(isize, isize, Direction), isize>,
        end: (isize, isize),
    ) -> isize {
        let mut min_price = isize::MAX;
        for dir in &Self::DIRECTIONS {
            if let Some(&cost) = visited.get(&(end.0, end.1, *dir)) {
                min_price = min_price.min(cost);
            }
        }
        min_price
    }

    fn find_paths(
        matrix: &Vec<Vec<char>>,
        start: (isize, isize),
    ) -> (
        Vec<(isize, Vec<(isize, isize, Direction)>)>,
        HashMap<(isize, isize, Direction), isize>,
    ) {
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        let mut paths = Vec::new();

        queue.push_back((vec![(start.0, start.1, Direction::Right)], 0));
        visited.insert((start.0, start.1, Direction::Right), 0);

        while let Some((path, price)) = queue.pop_front() {
            let (row, col, direction) = path.last().unwrap();

            if !in_bounds(matrix, *row, *col) || matrix[*row as usize][*col as usize] == '#' {
                continue;
            }

            if matrix[*row as usize][*col as usize] == 'E' {
                paths.push((price, path.clone()));
                continue;
            }

            for &(next_dir, row_offset, col_offset) in &Direction::offsets() {
                let new_row = row + row_offset;
                let new_col = col + col_offset;
                let new_price = price + if next_dir == *direction { 1 } else { 1001 };

                if visited
                    .get(&(new_row, new_col, next_dir))
                    .map_or(true, |&existing_price| new_price <= existing_price)
                {
                    let mut new_path = path.clone();
                    new_path.push((new_row, new_col, next_dir));

                    queue.push_back((new_path, new_price));
                    visited.insert((new_row, new_col, next_dir), new_price);
                }
            }
        }

        (paths, visited)
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
