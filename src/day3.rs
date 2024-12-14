const MUL_PATTERN: [char; 3] = ['m', 'u', 'l'];
const DO_PATTERN: [char; 4] = ['d', 'o', '(', ')'];
const DONT_PATTERN: [char; 7] = ['d', 'o', 'n', '\'', 't', '(', ')'];

#[derive(PartialEq, Eq)]
pub enum Instruction {
    Do,
    Dont,
}

pub struct AocDay3 {
    input: Vec<String>,
    instruction: Instruction,
}

impl AocDay3 {
    pub fn new(input: Vec<String>) -> AocDay3 {
        AocDay3 {
            input,
            instruction: Instruction::Do,
        }
    }

    pub fn part1(&mut self) -> i32 {
        self.input.iter().fold(0, |acc, line| {
            let chars = line.chars().collect::<Vec<_>>();
            acc + chars.iter().enumerate().fold(0, |acc, (i, c)| {
                if *c == 'm' {
                    acc + Self::eval_mul(&chars, i)
                } else {
                    acc
                }
            })
        })
    }

    pub fn part2(&mut self) -> i32 {
        self.input.iter().fold(0, |acc, line| {
            let chars = line.chars().collect::<Vec<_>>();
            let mut sum = 0;
            for (j, c) in chars.iter().enumerate() {
                if *c == 'm' {
                    if self.instruction == Instruction::Do {
                        sum += Self::eval_mul(&chars, j);
                    }
                }
                if *c == 'd' {
                    if let Some(instruction) = Self::parse_instruction(&chars, j) {
                        self.instruction = instruction;
                    }
                }
            }
            acc + sum
        })
    }

    fn eval_mul(chars: &[char], start: usize) -> i32 {
        if Self::matches_pattern(chars, start, &MUL_PATTERN) {
            if let Some((x, y)) = Self::get_params(chars, start + MUL_PATTERN.len()) {
                return x * y;
            }
        }
        0
    }

    fn matches_pattern(chars: &[char], start: usize, pattern: &[char]) -> bool {
        pattern
            .iter()
            .enumerate()
            .all(|(i, c)| chars.get(start + i) == Some(c))
    }

    fn get_params(chars: &[char], start: usize) -> Option<(i32, i32)> {
        let mut end = start + 1;
        if chars.get(start)? == &'(' {
            while let Some(c) = chars.get(end) {
                if *c == ')' {
                    let str = chars[start + 1..end].iter().collect::<String>();
                    return Self::parse_param_values(&str);
                }
                end += 1;
            }
        }
        None
    }

    fn parse_param_values(input: &str) -> Option<(i32, i32)> {
        let mut nums = input.split(',').map(str::parse::<i32>);
        if let (Some(Ok(x)), Some(Ok(y))) = (nums.next(), nums.next()) {
            Some((x, y))
        } else {
            None
        }
    }

    fn parse_instruction(chars: &[char], start: usize) -> Option<Instruction> {
        if Self::matches_pattern(chars, start, &DO_PATTERN) {
            Some(Instruction::Do)
        } else if Self::matches_pattern(chars, start, &DONT_PATTERN) {
            Some(Instruction::Dont)
        } else {
            None
        }
    }
}
