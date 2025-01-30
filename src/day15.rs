use crate::{
    utils::{in_bounds, read_lines},
    AocDay,
};
use std::{cell::RefCell, rc::Rc};

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

struct Node {
    left: (usize, usize),
    right: (usize, usize),
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(left: (usize, usize), right: (usize, usize)) -> Rc<Node> {
        Rc::new(Node {
            left,
            right,
            children: RefCell::new(Vec::new()),
        })
    }

    fn add_child(&self, child: Rc<Node>) {
        let mut children = self.children.borrow_mut();
        if !children
            .iter()
            .any(|c| c.left == child.left && c.right == child.right)
        {
            children.push(child);
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

    pub fn part2(&self) -> usize {
        let mut map = resize_map(&self.map);
        let mut robot_pos = get_robot_position(&map);

        for direction in self.directions.iter() {
            robot_pos = move_robot(&mut map, &direction, &robot_pos);
        }

        sum_coordinates(&map, '[')
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
        '[' | ']' => {
            let moved = shift_extended(map, pos, direction);
            if !moved {
                return pos.clone();
            }
            map[next.r][next.c] = '@';
            map[pos.r][pos.c] = '.';
            return next;
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

fn shift_extended(map: &mut Vec<Vec<char>>, robot: &Position, direction: &Direction) -> bool {
    match direction {
        Direction::Up | Direction::Down => {
            let root = build_tree(map, robot, direction);
            if check_leaves_movable(map, &root, *direction) {
                let mut old_pos = Vec::new();
                let mut new_pos = Vec::new();

                move_tree(map, &root, *direction, &mut old_pos, &mut new_pos);

                for old in old_pos.iter() {
                    map[old.0.r][old.0.c] = '.';
                    map[old.1.r][old.1.c] = '.';
                }

                for new in new_pos.iter() {
                    map[new.0.r][new.0.c] = '[';
                    map[new.1.r][new.1.c] = ']';
                }

                return true;
            }
        }
        Direction::Left | Direction::Right => {
            let step = match direction {
                Direction::Left => -1,
                Direction::Right => 1,
                _ => 0,
            };

            let next_pos = robot.next(direction);
            let mut next_c = next_pos.c as isize;
            let mut can_shift = false;

            while in_bounds(map, next_pos.r as isize, next_c)
                && map[next_pos.r][next_c as usize] != '#'
            {
                if map[next_pos.r][next_c as usize] == '.' {
                    can_shift = true;
                    break;
                }
                next_c += step;
            }

            if can_shift {
                match direction {
                    Direction::Left => {
                        for c in next_c as usize + 1..robot.c {
                            swap(map, *direction, robot.r as isize, c as isize);
                        }
                    }
                    Direction::Right => {
                        for c in (robot.c + 1..next_c as usize).rev() {
                            swap(map, *direction, robot.r as isize, c as isize);
                        }
                    }
                    _ => {}
                }

                return true;
            }
        }
    }

    false
}

fn build_tree(map: &Vec<Vec<char>>, robot: &Position, direction: &Direction) -> Rc<Node> {
    let next_pos = robot.next(direction);
    let (left, right) = get_box_position(map, next_pos.r, next_pos.c);

    let node = Node::new((left.r, left.c), (right.r, right.c));

    let mut queue = Vec::new();
    queue.push(Rc::clone(&node));

    while let Some(current) = queue.pop() {
        for position in [current.left, current.right].iter() {
            let current_pos = Position::new(position.0, position.1);
            let next = current_pos.next(direction);

            if is_box(map, next.r, next.c) {
                let (left, right) = get_box_position(map, next.r, next.c);
                let new_node = Node::new((left.r, left.c), (right.r, right.c));
                current.add_child(Rc::clone(&new_node));
                queue.push(new_node);
            }
        }
    }

    node
}

fn get_box_position(map: &Vec<Vec<char>>, row: usize, col: usize) -> (Position, Position) {
    if map[row][col] == '[' {
        return (Position::new(row, col), Position::new(row, col + 1));
    }
    (Position::new(row, col - 1), Position::new(row, col))
}

fn check_leaves_movable(map: &Vec<Vec<char>>, node: &Rc<Node>, direction: Direction) -> bool {
    if !has_free_space(map, node, &direction) {
        return false;
    }

    for child in node.children.borrow().iter() {
        if !check_leaves_movable(map, child, direction) {
            return false;
        }
    }

    true
}

fn has_free_space(map: &Vec<Vec<char>>, node: &Node, direction: &Direction) -> bool {
    let (start_r, start_c_left, start_c_right) = match direction {
        Direction::Up => (node.left.0.saturating_sub(1), node.left.1, node.right.1),
        Direction::Down => (node.left.0 + 1, node.left.1, node.right.1),
        _ => return false,
    };

    (start_c_left..=start_c_right).all(|c| is_free(map, start_r, c))
}

fn move_tree(
    map: &mut Vec<Vec<char>>,
    node: &Rc<Node>,
    direction: Direction,
    old_positions: &mut Vec<(Position, Position)>,
    new_positions: &mut Vec<(Position, Position)>,
) {
    for child in node.children.borrow().iter() {
        move_tree(map, child, direction, old_positions, new_positions);
    }

    let left = Position::new(node.left.0, node.left.1).next(&direction);
    let right = Position::new(node.right.0, node.right.1).next(&direction);

    new_positions.push((left, right));
    old_positions.push((
        Position::new(node.left.0, node.left.1),
        Position::new(node.right.0, node.right.1),
    ));
}

fn is_box(map: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if !in_bounds(map, row as isize, col as isize) {
        return false;
    }

    if map[row][col] == '[' || map[row][col] == ']' {
        return true;
    }

    false
}

fn is_free(map: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if in_bounds(map, row as isize, col as isize) && map[row][col] != '#' {
        return true;
    }

    false
}

fn swap(map: &mut Vec<Vec<char>>, d: Direction, r: isize, c: isize) {
    let (new_r, new_c) = match d {
        Direction::Left => (r, c - 1),
        Direction::Right => (r, c + 1),
        Direction::Up => (r - 1, c),
        Direction::Down => (r + 1, c),
    };

    let temp = map[r as usize][c as usize];
    map[r as usize][c as usize] = map[new_r as usize][new_c as usize];
    map[new_r as usize][new_c as usize] = temp;
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

fn resize_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = vec![vec!['.'; map[0].len() * 2]; map.len()];

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            let new_c = c * 2;

            if map[r][c] == '#' {
                new_map[r][new_c] = '#';
                new_map[r][new_c + 1] = '#';
            }

            if map[r][c] == 'O' {
                new_map[r][new_c] = '[';
                new_map[r][new_c + 1] = ']';
            }

            if map[r][c] == '@' {
                new_map[r][new_c] = '@';
                new_map[r][new_c + 1] = '.';
            }

            if map[r][c] == '.' {
                new_map[r][new_c] = '.';
                new_map[r][new_c + 1] = '.';
            }
        }
    }

    new_map
}

pub fn parse_input() -> (Vec<Vec<char>>, Vec<Direction>) {
    let input = read_lines("input/day15.txt");
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

impl AocDay for AocDay15 {
    fn part1(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part1())
    }

    fn part2(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part2())
    }
}
