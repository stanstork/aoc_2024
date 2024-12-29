use crate::utils::read_matrix;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
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
        let matrix = read_matrix("data/day16.txt");
        AocDay16 { matrix }
    }

    pub fn part1(&self) -> isize {
        let (start, end) = Self::find_start_end(&self.matrix);
        let visited = Self::dijkstra(&self.matrix, (end.0 as isize, end.1 as isize));

        visited[&((start.0 as isize, start.1 as isize), Direction::Right)]
    }

    pub fn part2(&self) -> usize {
        let mut matrix = self.matrix.clone();
        let (start, end) = Self::find_start_end(&matrix);
        let visited = Self::dijkstra(&matrix, (end.0 as isize, end.1 as isize));

        Self::find_best_paths(&mut matrix, (start.0, start.1), visited)
    }

    fn dijkstra(
        matrix: &Vec<Vec<char>>,
        end: (isize, isize),
    ) -> HashMap<((isize, isize), Direction), isize> {
        let mut visited = HashMap::new();
        let mut queue = BinaryHeap::new();

        for &dir in &Self::DIRECTIONS {
            let pos = (end, dir);
            queue.push((0, pos));
            visited.insert(pos, 0);
        }

        while let Some((price, pos)) = queue.pop() {
            for (next, next_price) in Self::make_move(matrix, pos, false) {
                let new_price = price + next_price;
                if new_price < *visited.get(&next).unwrap_or(&isize::MAX) {
                    visited.insert(next, new_price);
                    queue.push((new_price, next));
                }
            }
        }

        visited
    }

    fn make_move(
        matrix: &Vec<Vec<char>>,
        pos: ((isize, isize), Direction),
        forward: bool,
    ) -> Vec<(((isize, isize), Direction), isize)> {
        let mut results = Vec::new();
        for &dir in &Self::DIRECTIONS {
            let offset = dir.offset();
            let new_pos = if forward {
                (pos.0 .0 + offset.0, pos.0 .1 + offset.1)
            } else {
                (pos.0 .0 - offset.0, pos.0 .1 - offset.1)
            };
            if matrix[new_pos.0 as usize][new_pos.1 as usize] != '#' {
                results.push(((new_pos, dir), if dir == pos.1 { 1 } else { 1001 }));
            }
            if dir != pos.1 && dir != pos.1.opposite() {
                results.push(((pos.0, dir), 1000));
            }
        }
        results
    }

    fn find_best_paths(
        matrix: &mut Vec<Vec<char>>,
        start: (usize, usize),
        visited: HashMap<((isize, isize), Direction), isize>,
    ) -> usize {
        let start = ((start.0 as isize, start.1 as isize), Direction::Right);
        let mut queue = BinaryHeap::new();

        queue.push((visited[&start], start));

        let mut paths = HashSet::new();
        paths.insert(start);

        while let Some((remaining, pos)) = queue.pop() {
            for (next, price) in Self::make_move(matrix, pos, true) {
                let next_remaining = remaining - price;
                if !paths.contains(&next) && visited[&next] == next_remaining {
                    paths.insert(next);
                    queue.push((next_remaining, next));
                }
            }
        }

        for (pos, _) in paths.iter() {
            matrix[pos.0 as usize][pos.1 as usize] = 'O';
        }

        matrix.iter().flatten().filter(|&&c| c == 'O').count()
    }

    fn find_start_end(matrix: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
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

        (start, end)
    }
}
