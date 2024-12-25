use crate::utils::{in_bounds, read_lines};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub r: usize,
    pub c: usize,
}

impl Position {
    pub fn new(r: usize, c: usize) -> Position {
        Position { r, c }
    }

    pub fn next(&self, direction: &Direction) -> Position {
        match direction {
            Direction::Up => Position {
                r: self.r - 1,
                c: self.c,
            },
            Direction::Down => Position {
                r: self.r + 1,
                c: self.c,
            },
            Direction::Left => Position {
                r: self.r,
                c: self.c - 1,
            },
            Direction::Right => Position {
                r: self.r,
                c: self.c + 1,
            },
        }
    }
}

pub struct AocDay15 {
    pub map: Vec<Vec<char>>,
    pub directions: Vec<Direction>,
}

impl AocDay15 {
    pub fn new() -> AocDay15 {
        let (map, directions) = parse_input();
        AocDay15 { map, directions }
    }

    pub fn part1(&self) -> usize {
        let mut map = self.map.clone();
        let mut robot_pos = get_robot_position(&map);

        for d in self.directions.iter() {
            let next_pos = robot_pos.next(&d);
            if in_bounds(&map, next_pos.r as isize, next_pos.c as isize) {
                robot_pos = move_robot(&mut map, d, &robot_pos);
            }
        }

        sum_coordinates(&map, 'O')
    }
}

fn move_robot(map: &mut Vec<Vec<char>>, direction: &Direction, pos: &Position) -> Position {
    let next = pos.next(direction);

    match map[next.r][next.c] {
        '#' => return pos.clone(),
        'O' => {
            if shift(map, direction, next.r as isize, next.c as isize) {
                return next;
            } else {
                return pos.clone();
            }
        }
        _ => {
            map[pos.r][pos.c] = '.';
            map[next.r][next.c] = '@';
            return next;
        }
    }
}
fn shift(map: &mut Vec<Vec<char>>, direction: &Direction, r: isize, c: isize) -> bool {
    let (mut new_r, mut new_c) = (r, c);
    let mut can_shift = false;

    while in_bounds(map, new_r, new_c) {
        match map[new_r as usize][new_c as usize] {
            '#' => break,
            '.' => {
                can_shift = true;
                break;
            }
            _ => {}
        }

        match direction {
            Direction::Left => new_c -= 1,
            Direction::Right => new_c += 1,
            Direction::Up => new_r -= 1,
            Direction::Down => new_r += 1,
        }
    }

    if !can_shift {
        return false;
    }

    match direction {
        Direction::Left => {
            for c in new_c..c {
                map[r as usize][c as usize] = 'O';
            }
        }
        Direction::Right => {
            for c in c..=new_c {
                map[r as usize][c as usize] = 'O';
            }
        }
        Direction::Up => {
            for r in new_r..r {
                map[r as usize][c as usize] = 'O';
            }
        }
        Direction::Down => {
            for r in r..=new_r {
                map[r as usize][c as usize] = 'O';
            }
        }
    }

    map[r as usize][c as usize] = match direction {
        Direction::Left => {
            map[r as usize][c as usize + 1] = '.';
            '@'
        }
        Direction::Right => {
            map[r as usize][c as usize - 1] = '.';
            '@'
        }
        Direction::Up => {
            map[r as usize + 1][c as usize] = '.';
            '@'
        }
        Direction::Down => {
            map[r as usize - 1][c as usize] = '.';
            '@'
        }
    };

    true
}

pub fn sum_coordinates(map: &Vec<Vec<char>>, target: char) -> usize {
    let mut sum = 0;
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == target {
                sum += 100 * r + c;
            }
        }
    }

    sum
}

pub fn get_robot_position(map: &Vec<Vec<char>>) -> Position {
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == '@' {
                return Position { r, c };
            }
        }
    }

    panic!("Robot not found");
}

pub fn parse_input() -> (Vec<Vec<char>>, Vec<Direction>) {
    let input = read_lines("data/day15.txt");
    let empty_line_index = input
        .iter()
        .position(|line| line.is_empty())
        .expect("Invalid input");

    let map = input
        .iter()
        .take(empty_line_index)
        .map(|line| line.chars().collect())
        .collect();

    let directions = input
        .iter()
        .skip(empty_line_index + 1)
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err("Invalid direction character"),
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("Invalid directions");

    (map, directions)
}
