use std::{
    collections::{HashMap, HashSet},
    vec,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position(isize, isize);

pub struct AocDay21 {
    codes: Vec<String>,
    num_keypad: HashMap<char, Position>,
    dir_keypad: HashMap<char, Position>,
}

impl AocDay21 {
    pub fn new() -> Self {
        let lines = crate::utils::read_lines("data/day21.txt");
        AocDay21 {
            codes: lines,
            num_keypad: HashMap::from([
                ('7', Position(0, 0)),
                ('8', Position(0, 1)),
                ('9', Position(0, 2)),
                ('4', Position(1, 0)),
                ('5', Position(1, 1)),
                ('6', Position(1, 2)),
                ('1', Position(2, 0)),
                ('2', Position(2, 1)),
                ('3', Position(2, 2)),
                ('0', Position(3, 1)),
                ('A', Position(3, 2)),
            ]),
            dir_keypad: HashMap::from([
                ('^', Position(0, 1)),
                ('A', Position(0, 2)),
                ('<', Position(1, 0)),
                ('v', Position(1, 1)),
                ('>', Position(1, 2)),
            ]),
        }
    }

    pub fn part1(&self) -> isize {
        self.calc_code_len(2)
    }

    pub fn part2(&self) -> isize {
        self.calc_code_len(25)
    }

    fn calc_code_len(&self, level: usize) -> isize {
        let mut current_pos = *self.num_keypad.get(&'A').unwrap();
        let mut memo = HashMap::new();
        let mut total_len = 0;

        for code in self.codes.iter() {
            let mut code_len = 0;
            for c in code.chars() {
                let target_pos = self.num_keypad[&c];
                let paths = Self::get_paths(current_pos, self.num_keypad[&c], &self.num_keypad);
                let mut min_path_len = std::usize::MAX;

                for path in paths {
                    let path_len = Self::calc_level_len(&path, &self.dir_keypad, level, &mut memo);
                    min_path_len = std::cmp::min(min_path_len, path_len);
                }

                code_len += min_path_len;
                current_pos = target_pos;
            }

            total_len +=
                code_len as isize * Self::extract_num_part(&code.chars().collect::<Vec<char>>());
        }

        total_len
    }

    fn calc_level_len(
        path: &Vec<char>,
        keypad: &HashMap<char, Position>,
        level: usize,
        memo: &mut HashMap<(Vec<char>, usize), usize>,
    ) -> usize {
        if level == 0 {
            return path.len();
        }

        if let Some(cached) = memo.get(&(path.clone(), level)) {
            return *cached;
        }

        let mut current_pos = *keypad.get(&'A').unwrap();
        let mut code_len = 0;

        for c in path {
            let target_pos = keypad[&c];
            let paths = Self::get_paths(current_pos, keypad[&c], &keypad);
            let mut min_len = std::usize::MAX;

            for path in &paths {
                let path_len = Self::calc_level_len(path, keypad, level - 1, memo);
                min_len = std::cmp::min(min_len, path_len);
            }

            code_len += min_len;
            current_pos = target_pos;
        }

        memo.insert((path.clone(), level), code_len);
        code_len
    }

    fn get_paths(
        start: Position,
        target: Position,
        keypad: &HashMap<char, Position>,
    ) -> Vec<Vec<char>> {
        if start == target {
            return vec![vec!['A']];
        }

        let mut path = vec![];
        let mut paths = vec![];
        let mut visited = HashSet::new();

        fn dfs(
            keypad: &HashMap<char, Position>,
            current: Position,
            target: Position,
            path: &mut Vec<char>,
            paths: &mut Vec<Vec<char>>,
            visited: &mut HashSet<Position>,
        ) {
            if current == target {
                let mut final_path = path.clone();
                final_path.push('A');
                paths.push(final_path);
                return;
            }

            if visited.contains(&current) {
                return;
            }

            visited.insert(current);

            for &(dr, dc, dir) in &[(0, 1, '>'), (0, -1, '<'), (1, 0, 'v'), (-1, 0, '^')] {
                let new_pos = Position(current.0 + dr, current.1 + dc);
                if keypad.values().any(|&pos| pos == new_pos) {
                    path.push(dir);
                    dfs(keypad, new_pos, target, path, paths, visited);
                    path.pop();
                }
            }

            visited.remove(&current);
        }

        dfs(keypad, start, target, &mut path, &mut paths, &mut visited);
        paths
    }

    fn extract_num_part(code: &Vec<char>) -> isize {
        let numeric_part: String = code.iter().filter(|c| c.is_numeric()).collect();
        numeric_part.parse().unwrap()
    }
}
