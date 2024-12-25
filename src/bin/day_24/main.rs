use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::BitXor;
use std::sync::LazyLock;
use itertools::Itertools;
use regex::Regex;

static RE_WIRE: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"(.{3}): (\d)").unwrap()
);

static RE_BINOP: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"(.{3}) (\w+) (.{3}) -> (.{3})").unwrap()
);


type Wires = HashMap<String, u64>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum BinOp {
    And,
    Or,
    Xor,
}

impl BinOp {
    fn eval(&self, a: u64, b: u64) -> u64 {
        match self {
            BinOp::And => a & b,
            BinOp::Or => a | b,
            BinOp::Xor => a ^ b,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Op {
    op: BinOp,
    a: String,
    b: String,
    c: String,
}


#[derive(Clone, Debug, Eq, PartialEq)]
struct Device {
    wires: Wires,
    ops_map: HashMap<String, Op>,
}

impl Device {
    fn wires_to_binary_vec(&self, wires: Vec<String>) -> Vec<u64> {
        wires.iter().rev().map(|wire| {
            *self.wires.get(wire).unwrap()
        }).collect()
    }

    fn binary_vec_to_u64(&self, vec: Vec<u64>) -> u64 {
        let mut out = 0u64;
        for (i, bit) in vec.iter().rev().enumerate() {
            out |= *bit << i;
        }
        out
    }

    fn wires_to_u64(&self, wires: Vec<String>) -> u64 {
        self.binary_vec_to_u64(self.wires_to_binary_vec(wires))
    }

    fn wires_str_to_u64(&self, c: String) -> u64 {
        let mut x_wires = self.wires.keys()
            .filter(|&k| k.starts_with(&c))
            .map(|op| op.clone())
            .collect_vec();
        x_wires.sort_by(|a, b| a.cmp(b));
        self.wires_to_u64(x_wires)
    }


    fn eval(&mut self) -> u64 {
        let mut z_wires = self.ops_map
            .keys()
            .filter(|&k| k.starts_with("z"))
            .map(|op| op.clone())
            .collect_vec();
        z_wires.sort_by(|a, b| a.cmp(b));

        for result_wire in z_wires.clone() {
            self.eval_op(&self.ops_map.get(&result_wire).unwrap().clone(), &mut HashSet::new());
        }
        self.wires_str_to_u64("z".to_string())
    }

    fn eval_op(&mut self, op: &Op, seen: &mut HashSet<Op>) -> Option<u64> {
        // more complex than needed from testing part 2 stuff out, need to clean up
        if seen.contains(op) {
            return None;
        }
        seen.insert(op.clone());
        let a = if let Some(&a) = self.wires.get(&op.a) {
            a
        } else {
            let op_for_a = self.ops_map.get(&op.a).unwrap().clone();
            let a = self.eval_op(&op_for_a, seen);
            if a.is_none() {
                return None;
            }
            a.unwrap()
        };
        let b = if let Some(&b) = self.wires.get(&op.b) {
            b
        } else {
            let op_for_b = self.ops_map.get(&op.b).unwrap().clone();
            let b = self.eval_op(&op_for_b, seen);
            if b.is_none() {
                return None;
            }
            b.unwrap()
        };
        let c = op.op.eval(a, b);
        self.wires.insert(op.c.clone(), c);
        Some(c)
    }

    // fn find_dependencies(&self, wire: &str) -> HashSet<Op> {
    //     let mut deps = HashSet::new();
    //     let mut queue = VecDeque::new();
    //     queue.push_back(wire.to_string());
    //     while let Some(wire) = queue.pop_front() {
    //         if let Some(op) = self.ops_map.get(&wire) {
    //             if !deps.contains(op) {
    //                 deps.insert(op.clone());
    //                 queue.push_back(op.a.clone());
    //                 queue.push_back(op.b.clone());
    //             }
    //         }
    //     }
    //     deps
    // }

    fn swap(&mut self, a: String, b: String) {
        let a_op = self.ops_map.get(&a).unwrap().clone();
        let b_op = self.ops_map.get(&b).unwrap().clone();
        let mut a_op_next = a_op.clone();
        let mut b_op_next = b_op.clone();
        a_op_next.c = b_op.c.clone();
        b_op_next.c = a_op.c.clone();
        self.ops_map.insert(a_op_next.c.clone(), a_op_next);
        self.ops_map.insert(b_op_next.c.clone(), b_op_next);
    }
}

impl From<&str> for Device {
    fn from(value: &str) -> Self {
        let split = value.split("\n\n");
        let (wires_str, ops_str) = split.collect_tuple().unwrap();
        let mut wires = Wires::new();

        for wire in wires_str.lines() {
            let c = RE_WIRE.captures(wire).unwrap();
            let (_, [name, value]) = c.extract();
            let value = value.parse().unwrap();
            wires.insert(name.to_string(), value);
        }

        let mut ops: HashMap<String, Op> = HashMap::new();
        for op in ops_str.lines() {
            let c = RE_BINOP.captures(op).unwrap();
            let (_, [a, o, b, c]) = c.extract();
            match o {
                "AND" => {
                    ops.insert(c.to_string(), Op {
                        op: BinOp::And,
                        a: a.to_string(),
                        b: b.to_string(),
                        c: c.to_string(),
                    });
                }
                "OR" => {
                    // println!("{:?}", [a, o, b, c]);
                    ops.insert(c.to_string(), Op {
                        op: BinOp::Or,
                        a: a.to_string(),
                        b: b.to_string(),
                        c: c.to_string(),
                    });
                }
                "XOR" => {
                    ops.insert(c.to_string(), Op {
                        op: BinOp::Xor,
                        a: a.to_string(),
                        b: b.to_string(),
                        c: c.to_string(),
                    });
                }
                _ => panic!("Unknown operation: {}", op)
            }
        }
        Device { wires, ops_map: ops }
    }
}

fn main() {
    let input = include_str!("input");
    let mut base_device: Device = input.into();
    let mut device = base_device.clone();
    let n = device.eval();
    println!("Part 1: {:?}", n);

    // was able to figure out part 2 by hand, need to do by code
    // let x_wires_n = device.wires_str_to_u64("x".to_string());
    // let y_wires_n = device.wires_str_to_u64("y".to_string());
    // println!("{:#064b}", x_wires_n);
    // println!("{:#064b}", y_wires_n);
    // let sum = x_wires_n + y_wires_n;
    // device.swap("z11".to_string(), "wpd".to_string());
    // device.swap("skh".to_string(), "jqf".to_string());
    // device.swap("z19".to_string(), "cmp".to_string());
    // device.swap("rhh".to_string(), "wts".to_string());
    // let mut v = vec![
    //     "z11".to_string(), "wpd".to_string(),
    //     "skh".to_string(), "jqf".to_string(),
    //     "z19".to_string(), "cmp".to_string(),
    //     "rhh".to_string(), "wts".to_string(),
    // ];
    // v.sort();
    // println!("{}", v.join(","));
}