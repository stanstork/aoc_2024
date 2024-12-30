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
            let operand = self.get_operand_value(self.instructions[self.pc + 1], opcode);
            match opcode {
                OpCode::Adv | OpCode::Bdv | OpCode::Cdv => {
                    self.division(opcode, operand);
                }
                OpCode::Bxl => {
                    self.registers[1].value ^= operand;
                }
                OpCode::Bst | OpCode::Out => {
                    let result = operand % 8;
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

    fn get_operand_value(&self, operand: isize, opcode: OpCode) -> isize {
        match opcode {
            OpCode::Adv | OpCode::Bdv | OpCode::Cdv | OpCode::Out => match operand {
                0..=3 => operand,
                4 => self.registers[0].value,
                5 => self.registers[1].value,
                6 => self.registers[2].value,
                _ => panic!("Invalid combo operand"),
            },
            _ => operand,
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
}

pub fn part1() {
    let (instructions, a, b, c) = parse_program();

    println!("Instructions: {:?}", instructions);
    println!("Registers: {}, {}, {}", a, b, c);

    let mut computer = Computer::new(a, b, c, instructions);
    computer.run();

    println!("Output: {}", computer.output());
}

fn parse_program() -> (Vec<isize>, isize, isize, isize) {
    let input = utils::split_lines_whitespace("data/day17.txt");
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
