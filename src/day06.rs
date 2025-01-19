use std::collections::HashMap;
use crate::utils;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_coord(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

pub struct AocDay6 {
    map: Vec<Vec<char>>,
}

impl AocDay6 {
    pub fn new() -> Self {
        AocDay6 {
            map: utils::read_matrix("data/day6.txt"),
        }
    }

    pub fn part1(&self) -> i32 {
        let mut map = self.map.clone();
        let (mut x, mut y) = self.get_starting_point();
        let mut direction = Direction::Up;

        loop {
            let (next_x, next_y) = direction.next_coord(x, y);

            if Self::is_edge(&map, next_x, next_y) {
                map[y as usize][x as usize] = 'X';
                break;
            }

            if Self::is_obstruction(&map, next_x, next_y) {
                direction = direction.turn_right();
            } else {
                map[y as usize][x as usize] = 'X';
                x = next_x;
                y = next_y;
            }
        }

        map.iter().flatten().filter(|&c| *c == 'X').count() as i32
    }

    pub fn part2(&self) -> i32{
        let mut map = self.map.clone();
        let (start_x, start_y) = self.get_starting_point();

        let mut cycles = 0;
    
        for r in 0..map.len() {
            for c in 0..map[0].len() {
                if map[r][c] == '.' {
                    map[r][c] = '#';
    
                    if Self::detect_cycle(&map, start_x, start_y) {
                        cycles += 1;
                    }
    
                    map[r][c] = '.';
                }
            }
        }

        cycles
    }

    fn get_starting_point(&self) -> (i32, i32) {
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if self.map[y][x] == '^' {
                    return (x as i32, y as i32);
                }
            }
        }
        panic!("No starting point found");
    }

    fn is_edge(map: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || y >= map.len() as i32 || x >= map[0].len() as i32 {
            return true;
        }
        false
    }

    fn is_obstruction(map: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || y >= map.len() as i32 || x >= map[0].len() as i32 {
            return true;
        }
        map[y as usize][x as usize] == '#'
    }

    fn detect_cycle(map: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
        let (mut x, mut y) = (x, y);
        let mut direction = Direction::Up;
        let mut visit_counts: HashMap<((i32, i32), Direction), usize> = HashMap::new();

        loop {
            let (next_x, next_y) = direction.next_coord(x, y);

            if Self::is_edge(&map, next_x, next_y) {
                break;
            }

            if Self::is_obstruction(&map, next_x, next_y) {
                direction = direction.turn_right();
            } else {
                let entry = visit_counts
                    .entry(((next_x, next_y), direction))
                    .or_insert(0);
                *entry += 1;

                if *entry > 1 {
                    return true;
                }

                x = next_x;
                y = next_y;
            }
        }

        false
    }
}
