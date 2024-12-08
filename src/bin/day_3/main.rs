use regex::{Captures, Regex};
use std::sync::LazyLock;

static RE_INSTRUCTIONS: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"(?<mul>mul\(\d+,\d+\))|(?<do>do\(\))|(?<dont>don't\(\))"
    ).unwrap()
);

static RE_MUL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap()
);

enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

impl<'a> From<Captures<'a>> for Instruction {
    fn from(captures: Captures<'a>) -> Self {
        if let Some(mul) = captures.name("mul") {
            let Some(mul_captures) = RE_MUL.captures(mul.as_str()) else {
                panic!("Invalid 'mul' instruction: {}", mul.as_str());
            };
            let (_, [left, right]) = mul_captures.extract();
            Instruction::Mul(left.parse().unwrap(), right.parse().unwrap())
        } else if captures.name("do").is_some() {
            Instruction::Do
        } else if captures.name("dont").is_some() {
            Instruction::Dont
        } else {
            panic!("Invalid instruction: {}", captures.get(0).unwrap().as_str());
        }
    }
}

struct Computer<'a> {
    memory: &'a str,
}

impl<'a> Computer<'a> {
    fn find_instructions(&self) -> Vec<Instruction> {
        RE_INSTRUCTIONS
            .captures_iter(self.memory)
            .map(|captures| captures.into())
            .collect()
    }

    fn run_instructions(&self, instructions: &Vec<Instruction>, enable_dos_and_donts: bool) -> usize {
        let mut enable_mul = true;
        instructions.iter().filter_map(|instruction| {
            match instruction {
                Instruction::Mul(left, right) => {
                    if !enable_dos_and_donts || enable_mul {
                        return Some(left * right);
                    }
                }
                Instruction::Do => {
                    enable_mul = true;
                }
                Instruction::Dont => {
                    enable_mul = false;
                }
            };
            return None;
        }).sum()
    }
}

fn main() {
    let input = include_str!("input");
    let computer = Computer {
        memory: input
    };

    let instructions = computer.find_instructions();

    let output_without_dos_and_donts = computer.run_instructions(&instructions, false);
    println!("Part 1: {}", output_without_dos_and_donts);

    let output_with_dos_and_donts = computer.run_instructions(&instructions, true);
    println!("Part 2: {}", output_with_dos_and_donts);
}