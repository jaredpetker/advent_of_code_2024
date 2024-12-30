use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{BitXor, Rem};
use itertools::Itertools;
use advent_of_code_2024::Vec2D;

type Pos = Vec2D;

#[derive(Clone, Debug, Eq, PartialEq)]
struct SecretNumber(u64);

impl SecretNumber {
    fn evolve(&mut self) {
        let step_1 = Self::prune(Self::mix(self.0 * 64, self.0));
        let step_2 = Self::prune(Self::mix(step_1 / 32u64, step_1));
        self.0 = Self::prune(Self::mix(step_2, step_2 * 2048));
    }

    fn mix(from: u64, into: u64) -> u64 {
        from.bitxor(into)
    }

    fn prune(n: u64) -> u64 {
        n.rem_euclid(16777216u64)
    }
}

impl From<&str> for SecretNumber {
    fn from(value: &str) -> Self {
        SecretNumber(value.parse().unwrap())
    }
}

fn keyed(v: &Vec<i64>, evolution: usize) -> String {
    format!("{},{},{},{}", v[evolution - 3], v[evolution - 2], v[evolution - 1], v[evolution])
}

fn main() {
    let input = include_str!("input");
    let mut secret_numbers = input.lines().map(|l| SecretNumber::from(l)).collect_vec();
    let mut sum = 0;
    let mut h = HashMap::new();
    let sn_len = secret_numbers.len();
    for (i, secret_number) in secret_numbers.iter_mut().enumerate() {
        let mut left = secret_number.0.rem_euclid(10);
        let mut v = vec![];
        for evolution in 0..2000 {
            secret_number.evolve();
            let right = secret_number.0.rem_euclid(10);
            v.push(right as i64 - left as i64);

            if evolution >= 3 {
                let key = keyed(&v, evolution);
                if !h.contains_key(&key) {
                    h.insert(key.clone(), vec![u64::MIN; sn_len]);
                }
                if let Some(v) = h.get_mut(&key) {
                    if v[i] == u64::MIN {
                        v[i] = right;
                    }
                }
            }
            left = right;
        }
        sum += secret_number.0;
    }
    println!("Part 1: {}", sum);

    let h: HashMap<String, u64> = h.iter().map(|(k, v)| {
        (k.clone(), v.iter().sum::<u64>())
    }).collect();

    let Some(m) = h.iter().max_by(|a, b| a.1.cmp(b.1)) else {
        panic!("No max found");
    };

    println!("Part 2: {:?}", m.1);
}
