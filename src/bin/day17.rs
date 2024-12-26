use std::fmt::Display;

use itertools::Itertools;
use regex::Regex;
use rusty_advent_2024::utils::file_io;

type Number = u64;

enum Outcome {
    None,
    Halt,
    Output(Number),
}

#[derive(Clone)]
struct ProgramState {
    a: Number,
    b: Number,
    c: Number,
    program: Vec<u8>,
    instruction_ptr: usize,
}

impl Display for ProgramState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A: {}, B: {}, C: {}\n{}\n{} ",
            self.a,
            self.b,
            self.c,
            self.program.clone().into_iter().join(""),
            " ".repeat(self.instruction_ptr) + "^"
        )
    }
}

fn unique_match(haystack: &str, pattern: &str) -> String {
    Regex::new(pattern)
        .expect("Pattern must be a valid regex expression.")
        .captures(haystack)
        .expect("Pattern should match.")
        .extract::<1>()
        .1[0]
        .into()
}

fn parse_program_string(program_string: &str) -> Vec<u8> {
    program_string
        .split(',')
        .map(|s| s.parse().expect("Error parsing program input."))
        .collect()
}

//#[cfg(test)]
impl ProgramState {
    fn new(program_string: &str) -> Self {
        ProgramState {
            a: 0,
            b: 0,
            c: 0,
            instruction_ptr: 0,
            program: parse_program_string(program_string),
        }
    }

    fn set_a(mut self, a: Number) -> Self {
        self.a = a;
        self
    }
}

#[cfg(test)]
impl ProgramState {
    fn set_b(mut self, b: Number) -> Self {
        self.b = b;
        self
    }

    fn set_c(mut self, c: Number) -> Self {
        self.c = c;
        self
    }
}

impl ProgramState {
    fn from(data_string: &str) -> Self {
        ProgramState {
            a: unique_match(data_string, r"Register A: (.*)")
                .parse()
                .expect("Register A could not be parsed."),
            b: unique_match(data_string, r"Register B: (.*)")
                .parse()
                .expect("Register B could not be parsed."),
            c: unique_match(data_string, r"Register C: (.*)")
                .parse()
                .expect("Register C could not be parsed."),
            instruction_ptr: 0,
            program: parse_program_string(&unique_match(data_string, r"Program: (.*)")),
        }
    }

    fn combo(&self, operand: Number) -> Number {
        match operand {
            c if c < 4 => c as Number,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Combo value reserved - invalid program."),
        }
    }

    fn step(&mut self) -> Outcome {
        // take one step, optional output
        if self.instruction_ptr > self.program.len() - 2 {
            return Outcome::Halt;
        }

        let (instruction, operand) = (
            self.program[self.instruction_ptr],
            self.program[self.instruction_ptr + 1] as Number,
        );

        self.instruction_ptr += 2;

        match instruction {
            0 => self.a >>= self.combo(operand),
            1 => self.b ^= operand,
            2 => self.b = self.combo(operand) % 8,
            3 => {
                if self.a != 0 {
                    self.instruction_ptr = operand as usize
                }
            }
            4 => self.b ^= self.c,
            5 => return Outcome::Output(self.combo(operand) % 8),
            6 => self.b = self.a >> self.combo(operand),
            7 => self.c = self.a >> self.combo(operand),
            _ => panic!("Invalid instruction - bad program."),
        }

        Outcome::None
    }

    fn run(&mut self) -> String {
        let mut outputs = Vec::new();
        loop {
            match self.step() {
                Outcome::Output(out) => outputs.push(out),
                Outcome::Halt => break,
                _ => (),
            }
        }
        outputs.into_iter().join(",")
    }
}

fn load_program(path: &str) -> ProgramState {
    ProgramState::from(&file_io::strings_from_file(path).join("\n"))
}

fn reverse_engineer_a(
    program_string: &str,
    intended_output: &[u8],
    fixed_a: Number,
) -> Option<Number> {
    if intended_output.is_empty() {
        return Some(fixed_a);
    }
    let last_out = *intended_output.last().unwrap();

    for a in 0..8 {
        let new_a = (fixed_a << 3) + a;
        if new_a == 0 {
            // handle special case only relevant in first round
            continue;
        }
        let mut program = ProgramState::new(program_string).set_a(new_a);
        loop {
            match program.step() {
                Outcome::None => (),
                Outcome::Halt => break,
                Outcome::Output(out) => {
                    if out as u8 == last_out {
                        // try go deeper
                        if let Some(total_a) = reverse_engineer_a(
                            program_string,
                            &intended_output[0..intended_output.len() - 1],
                            new_a,
                        ) {
                            return Some(total_a);
                        }
                    }
                    break;
                }
            }
        }
    }

    None
}

fn part1(path: &str) -> String {
    let mut program = load_program(path);
    program.run()
}

fn part2(path: &str) -> Option<Number> {
    let program = load_program(path);
    let program_string = &program.program.clone().into_iter().join(",");
    let intended_output = program.program;
    reverse_engineer_a(program_string, &intended_output, 0)
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input17.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input17.txt").unwrap_or_default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiny_programs() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut prog1 = ProgramState::new("2,6").set_c(9);
        prog1.run();
        assert!(prog1.b == 1);
        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let mut prog2 = ProgramState::new("5,0,5,1,5,4").set_a(10);
        assert!(prog2.run() == "0,1,2");
        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        let mut prog3 = ProgramState::new("0,1,5,4,3,0").set_a(2024);
        assert!(prog3.run() == "4,2,5,6,7,7,7,7,3,1,0");
        assert!(prog3.a == 0);
        // If register B contains 29, the program 1,7 would set register B to 26.
        let mut prog4 = ProgramState::new("1,7").set_b(29);
        prog4.run();
        assert!(prog4.b == 26);
        // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354
        let mut prog5 = ProgramState::new("4,0").set_b(2024).set_c(43690);
        prog5.run();
        assert!(prog5.b == 44354);
    }

    #[test]
    fn test_part1() {
        assert!(part1("input/input17.txt.test1") == "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        assert!(part2("input/input17.txt.test2") == Some(117440))
    }
}
