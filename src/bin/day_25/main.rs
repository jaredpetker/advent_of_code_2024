use itertools::Itertools;


#[derive(Clone, Debug, Eq, PartialEq)]
struct Lock(Vec<i32>);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Key(Vec<i32>);

impl Key {
    fn can_unlock(&self, lock: &Lock) -> bool {
        self.0.iter().zip(lock.0.iter()).all(|(k, l)| k + l <= 5)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Schematic {
    Lock(Lock),
    Key(Key),
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        let mut is_key = false;
        let mut is_lock = false;
        let mut heights: Vec<i32> = vec![];
        for (_, line) in value.lines().enumerate() {
            if !is_lock && !is_key {
                is_lock = line.chars().next().unwrap() == '#';
                is_key = !is_lock;
            }
            for (x, c) in line.chars().enumerate() {
                if heights.len() <= x {
                    heights.push(-1);
                }
                if c == '#' {
                    heights[x] += 1;
                }
            }
        }
        if is_key {
            Schematic::Key(Key(heights))
        } else {
            Schematic::Lock(Lock(heights))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Schematics {
    keys: Vec<Key>,
    locks: Vec<Lock>,
}

impl Schematics {
    fn count_fits(&self) -> usize {
        self.keys.iter().map(|k| self.locks.iter().filter(|l| k.can_unlock(l)).count()).sum()
    }
}


impl From<&str> for Schematics {
    fn from(value: &str) -> Self {
        let schematics = value.split("\n\n").map(Schematic::from).collect_vec();
        Schematics {
            keys: schematics.iter().filter_map(|s| match s {
                Schematic::Key(k) => Some(k.clone()),
                _ => None,
            }).collect(),
            locks: schematics.iter().filter_map(|s| match s {
                Schematic::Lock(l) => Some(l.clone()),
                _ => None,
            }).collect(),
        }
    }
}


fn main() {
    let input = include_str!("input");
    let schematics = Schematics::from(input);
    println!("Part 1: {:?}", schematics.count_fits());
}