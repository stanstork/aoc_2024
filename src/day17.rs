use crate::utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl OpCode {
    fn from(opcode: isize) -> Self {
        match opcode {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => panic!("Invalid opcode"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Register {
    value: isize,
}

#[derive(Debug)]
struct Computer {
    registers: [Register; 3],
    instructions: Vec<isize>,
    out: Vec<isize>,
    pc: usize,
}

impl Computer {
    fn new(a: isize, b: isize, c: isize, instructions: Vec<isize>) -> Computer {
        Computer {
            registers: [
                Register { value: a },
                Register { value: b },
                Register { value: c },
            ],
            instructions,
            out: Vec::new(),
            pc: 0,
        }
    }

    fn run(&mut self) {
        while self.pc < self.instructions.len() {
            let opcode = OpCode::from(self.instructions[self.pc]);
            let operand = self.instructions[self.pc + 1];
            match opcode {
                OpCode::Adv | OpCode::Bdv | OpCode::Cdv => {
                    self.division(opcode, self.combo_operand(operand));
                }
                OpCode::Bxl => {
                    self.registers[1].value ^= operand;
                }
                OpCode::Bst | OpCode::Out => {
                    let result = self.combo_operand(operand) % 8;
                    if opcode == OpCode::Bst {
                        self.registers[1].value = result;
                    } else {
                        self.out.push(result);
                    }
                }
                OpCode::Jnz => {
                    if self.registers[0].value != 0 {
                        self.pc = operand as usize;
                        continue;
                    }
                }
                OpCode::Bxc => {
                    self.registers[1].value ^= self.registers[2].value;
                }
            }
            self.pc += 2;
        }
    }

    fn combo_operand(&self, operand: isize) -> isize {
        match operand {
            0..=3 => operand,
            4..=7 => self.registers[(operand - 4) as usize].value,
            _ => unreachable!(),
        }
    }

    fn division(&mut self, opcode: OpCode, operand: isize) {
        let result = self.registers[0].value >> operand;
        match opcode {
            OpCode::Adv => self.registers[0].value = result,
            OpCode::Bdv => self.registers[1].value = result,
            OpCode::Cdv => self.registers[2].value = result,
            _ => unreachable!(),
        }
    }

    fn output(&self) -> String {
        self.out
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }

    fn reset(&mut self) {
        self.registers[0].value = 0;
        self.registers[1].value = 0;
        self.registers[2].value = 0;
        self.out.clear();
        self.pc = 0;
    }

    fn set_registers(&mut self, a: isize, b: isize, c: isize) {
        self.registers[0].value = a;
        self.registers[1].value = b;
        self.registers[2].value = c;
    }
}

pub struct AocDay17 {
    instructions: Vec<isize>,
    a: isize,
    b: isize,
    c: isize,
}

impl AocDay17 {
    pub fn new() -> Self {
        let (instructions, a, b, c) = parse_program();
        AocDay17 {
            instructions,
            a,
            b,
            c,
        }
    }

    pub fn part1(&self) -> String {
        let mut computer = Computer::new(self.a, self.b, self.c, self.instructions.clone());
        computer.run();
        computer.output()
    }

    pub fn part2(&self) -> isize {
        let expected_out = self
            .instructions
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(",");

        let mut combinations = vec![0];
        let mut computer = Computer::new(0, self.b, self.c, self.instructions.clone());

        for _ in 0..self.instructions.len() {
            combinations = Self::filter_combinations(
                &combinations,
                &expected_out,
                &mut computer,
                self.b,
                self.c,
            );
        }

        *combinations.iter().min().unwrap()
    }

    fn filter_combinations(
        combinations: &Vec<isize>,
        expected_out: &str,
        computer: &mut Computer,
        b: isize,
        c: isize,
    ) -> Vec<isize> {
        let mut new_combinations = Vec::new();

        for &ah in combinations {
            for al in 0..8 {
                let a = ah * 8 + al;
                computer.reset();
                computer.set_registers(a, b, c);
                computer.run();

                if Self::is_valid_output(&computer.output(), expected_out) {
                    new_combinations.push(a);
                }
            }
        }

        new_combinations
    }

    fn is_valid_output(output: &str, instruction_str: &str) -> bool {
        let mut inst_idx = instruction_str.len() - 1;

        for ch in output.chars().rev() {
            if let Some(expected) = instruction_str.chars().nth(inst_idx) {
                if ch != expected {
                    return false;
                }
                inst_idx -= 1;
            } else {
                return false;
            }
        }

        true
    }
}

fn parse_program() -> (Vec<isize>, isize, isize, isize) {
    let input = utils::split_lines_whitespace("input/day17.txt");
    let registers = parse_registers(&input.0);
    let instructions = parse_instructions(&input.1[0]);
    (instructions, registers.0, registers.1, registers.2)
}

fn parse_registers(lines: &[String]) -> (isize, isize, isize) {
    let regs = lines
        .iter()
        .map(|line| {
            let parts = line.split(':').collect::<Vec<&str>>();
            parts[1].trim().parse::<isize>().unwrap()
        })
        .collect::<Vec<isize>>();

    (regs[0], regs[1], regs[2])
}

fn parse_instructions(line: &str) -> Vec<isize> {
    let parts = line.split(':').collect::<Vec<&str>>();
    let instr_line = parts[1].trim().split(',').collect::<Vec<&str>>();
    instr_line
        .iter()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}
