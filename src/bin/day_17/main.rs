use std::ops::{BitXor};
use std::sync::LazyLock;
use itertools::Itertools;
use regex::Regex;

static RE_REGISTER: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"Register \w: (\d+)").unwrap()
);

type Num = u64;

trait Combo {
    fn combo(&self, registers: &Registers) -> Self;
}

impl Combo for Num {
    fn combo(&self, registers: &Registers) -> Self {
        let n = *self;
        match n {
            0..=3 => n,
            4 => registers.a,
            5 => registers.b,
            6 => registers.c,
            _ => panic!("Invalid combo value: {}", n),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instr {
    Adv(Num),
    Bxl(Num),
    Bst(Num),
    Jnz(Num),
    Bxc(Num),
    Out(Num),
    Bdv(Num),
    Cdv(Num),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Register {
    A,
    B,
    C,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Registers {
    a: Num,
    b: Num,
    c: Num,
}

enum Result {
    Set(Register, Num),
    Continue,
    Jump(Num),
    Output(Num),
}

impl Instr {
    fn run(&self, registers: &mut Registers) -> Result {
        match self {
            Instr::Adv(n) => self.adv(*n, registers),
            Instr::Bxl(n) => self.bxl(*n, registers),
            Instr::Bst(n) => self.bst(*n, registers),
            Instr::Jnz(n) => self.jnz(*n, registers),
            Instr::Bxc(n) => self.bxc(*n, registers),
            Instr::Out(n) => self.out(*n, registers),
            Instr::Bdv(n) => self.bdv(*n, registers),
            Instr::Cdv(n) => self.cdv(*n, registers),
        }
    }

    fn adv(&self, n: Num, registers: &mut Registers) -> Result {
        Result::Set(Register::A, registers.a / 2_u64.pow(n.combo(registers) as u32))
    }

    fn bxl(&self, n: Num, registers: &mut Registers) -> Result {
        Result::Set(Register::B, registers.b.bitxor(n))
    }

    fn bst(&self, n: Num, registers: &mut Registers) -> Result {
        Result::Set(Register::B, n.combo(registers).rem_euclid(8))
    }

    fn jnz(&self, n: Num, registers: &mut Registers) -> Result {
        match registers.a {
            0 => Result::Continue,
            _ => Result::Jump(n),
        }
    }

    fn bxc(&self, _n: Num, registers: &mut Registers) -> Result {
        Result::Set(Register::B, registers.b.bitxor(registers.c))
    }

    fn out(&self, n: Num, registers: &mut Registers) -> Result {
        Result::Output(n.combo(registers).rem_euclid(8))
    }

    fn bdv(&self, n: Num, registers: &mut Registers) -> Result {
        Result::Set(Register::B, registers.a / 2_u64.pow(n.combo(registers) as u32))
    }

    fn cdv(&self, n: Num, registers: &mut Registers) -> Result {
        Result::Set(Register::C, registers.a / 2_u64.pow(n.combo(registers) as u32))
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
struct Computer {
    registers: Registers,
}

impl Computer {
    fn run_instr(&mut self, instr: &Instr) -> Result {
        instr.run(&mut self.registers)
    }

    fn run_program(&mut self, program: &Program) -> Vec<Num> {
        let program = program.0.clone();
        let mut i = 0;
        let mut output = vec![];
        while i < program.len() {
            let instr = &program[i];
            match self.run_instr(instr) {
                Result::Set(Register::A, n) => self.registers.a = n,
                Result::Set(Register::B, n) => self.registers.b = n,
                Result::Set(Register::C, n) => self.registers.c = n,
                Result::Continue => {}
                Result::Jump(n) => {
                    i = n as usize;
                    continue;
                }
                Result::Output(n) => output.push(n)
            }
            i += 1;
        }
        output
    }

    fn find_magic_register_a_value(&self, program: &Program) -> Num {
        // definitely not the "right" way to do this...
        let prog = program.1.clone();
        let mut curr = 0u64;
        for i in (1..prog.len()).rev() {
            let step = 8_u64.pow(i as u32);
            for j in (curr..).step_by(step as usize) {
                let mut cloned = self.clone();
                cloned.registers.a = j;
                let output = cloned.run_program(program);
                if output.len() != prog.len() {
                    continue;
                }
                if output[i] == prog[i] {
                    curr = j;
                    break;
                }
            }
        }

        for i in curr.. {
            let mut cloned = self.clone();
            cloned.registers.a = i;
            let output = cloned.run_program(program);
            if output == prog {
                return i;
            }
        }
        return 0;
    }
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        Computer {
            registers: Registers {
                a: RE_REGISTER.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse().unwrap(),
                b: RE_REGISTER.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse().unwrap(),
                c: RE_REGISTER.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse().unwrap(),
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Program(Vec<Instr>, Vec<Num>);


impl From<&str> for Program {
    fn from(value: &str) -> Self {
        let split = value.split_whitespace().collect_vec();
        let raw = split[1].split(",").map(|s| s.parse::<Num>().unwrap()).collect_vec();
        let chunks = split[1].split(',').chunks(2);
        Program(chunks.into_iter().map(|chunk| {
            let instr = chunk.collect_vec();
            match instr[0] {
                "0" => Instr::Adv(instr[1].parse().unwrap()),
                "1" => Instr::Bxl(instr[1].parse().unwrap()),
                "2" => Instr::Bst(instr[1].parse().unwrap()),
                "3" => Instr::Jnz(instr[1].parse().unwrap()),
                "4" => Instr::Bxc(instr[1].parse().unwrap()),
                "5" => Instr::Out(instr[1].parse().unwrap()),
                "6" => Instr::Bdv(instr[1].parse().unwrap()),
                "7" => Instr::Cdv(instr[1].parse().unwrap()),
                _ => panic!("Invalid instruction: {}", instr[0]),
            }
        }).collect(), raw)
    }
}


fn main() {
    let input = include_str!("input");
    let mut split = input.split("\n\n");
    let (mut computer, program) = (
        Computer::from(split.next().unwrap()), Program::from(split.next().unwrap())
    );
    println!("Part 1: {}", computer.run_program(&program).iter().join(","));
    println!("Part 2: {}", computer.find_magic_register_a_value(&program));
}