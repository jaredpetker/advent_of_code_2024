use itertools::{Itertools, repeat_n};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operator {
    Mul,
    Add,
    Concat,
}

impl Operator {
    fn compute(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
            Operator::Concat => format!("{}{}", lhs, rhs).parse().unwrap()
        }
    }
}

#[derive(Debug, Clone)]
struct Equation {
    test_value: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn solve<'a, T>(&self, ops: T) -> bool
    where
        T: Iterator<Item=&'a Operator> + Clone,
    {
        let op_permutations = repeat_n(ops, self.numbers.len() - 1).multi_cartesian_product();
        for op in op_permutations {
            let mut acc = self.numbers[0];
            for (i, num) in self.numbers.iter().enumerate().skip(1) {
                acc = op[i - 1].compute(acc, *num);
                if acc > self.test_value {
                    break;
                }
            }
            if acc == self.test_value {
                return true;
            }
        }
        return false;
    }
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let mut split = value.split(":");
        Equation {
            test_value: split.next().unwrap().parse().unwrap(),
            numbers: split.next().unwrap().trim().split(" ").map(|num| num.parse().unwrap()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct Calibration {
    equations: Vec<Equation>,
}

impl Calibration {
    fn sum_solvable_equations(&self, ops: &[Operator]) -> usize {
        self.equations.iter().filter_map(
            |equation| {
                if equation.solve(ops.into_iter()) {
                    Some(equation.test_value)
                } else {
                    None
                }
            }
        ).sum()
    }
}

impl From<&str> for Calibration {
    fn from(value: &str) -> Self {
        Calibration {
            equations: value.lines().map(|line| line.into()).collect()
        }
    }
}

fn main() {
    let input = include_str!("input");
    let calibration: Calibration = input.into();
    println!("Part 1: {}", calibration.sum_solvable_equations(&[Operator::Add, Operator::Mul]));
    println!("Part 2: {}", calibration.sum_solvable_equations(&[Operator::Add, Operator::Mul, Operator::Concat]));
}