use std::collections::{HashMap, HashSet};
use advent_of_code_2024::Vec2D;
use crate::Step::Invalid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Path {
    Clear,
    Obstacle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Valid,
    Invalid,
}

#[derive(Debug, Clone)]
struct Guard {
    pos: Vec2D,
    dir: Vec2D,
}

impl Guard {
    fn turn(&mut self) {
        self.dir = Vec2D { x: -self.dir.y, y: self.dir.x };
    }

    fn step(&mut self) {
        self.pos = self.pos + self.dir;
    }

    fn next_pos(&self) -> Vec2D {
        self.pos + self.dir
    }
}

struct Solution {
    distinct_visited_positions: usize,
    possible_obstruction_positions: usize,
}

#[derive(Debug, Clone)]
struct Lab {
    map: HashMap<Vec2D, Path>,
    guard: Guard,
    seen: HashMap<Vec2D, HashSet<Vec2D>>,
    possible_obstruction_positions: HashSet<Vec2D>,
}


impl Lab {
    fn step(&mut self) -> Step {
        let next_step = self.guard.next_pos();
        match self.map.get(&next_step) {
            Some(Path::Clear) => {
                self.guard.step();
                Step::Valid
            }
            Some(Path::Obstacle) => {
                self.guard.turn();
                Step::Valid
            }
            _ => Invalid
        }
    }

    fn solve(&mut self) -> Solution {
        loop {
            let next_pos = self.guard.next_pos();
            if Some(&Path::Clear) == self.map.get(&next_pos) && !self.seen.contains_key(&next_pos) {
                let mut new_map = self.clone();
                new_map.map.insert(next_pos, Path::Obstacle);
                loop {
                    new_map.seen.entry(new_map.guard.pos.clone()).or_default().insert(new_map.guard.dir.clone());
                    match new_map.step() {
                        Step::Valid => {
                            if new_map.seen.get(&new_map.guard.pos).is_some_and(|seen| seen.contains(&new_map.guard.dir)) {
                                self.possible_obstruction_positions.insert(next_pos);
                                break;
                            }
                        }
                        _ => break
                    }
                }
            }

            self.seen.entry(self.guard.pos.clone()).or_default().insert(self.guard.dir.clone());
            if self.step() == Invalid {
                break;
            }
        }
        Solution {
            distinct_visited_positions: self.seen.len(),
            possible_obstruction_positions: self.possible_obstruction_positions.len(),
        }
    }
}

impl From<&str> for Lab {
    fn from(value: &str) -> Self {
        let mut map = HashMap::new();
        let mut guard: Guard = Guard { pos: Vec2D { x: 0, y: 0 }, dir: Vec2D { x: 0, y: -1 } };
        let mut y = 0;
        for line in value.lines() {
            for (x, c) in line.chars().enumerate() {
                let pos = Vec2D { x: x as i64, y };
                match c {
                    '.' => { map.insert(pos, Path::Clear); }
                    '#' => { map.insert(pos, Path::Obstacle); }
                    '^' => {
                        guard.pos = pos;
                        map.insert(pos, Path::Clear);
                    }
                    c => panic!("Invalid character in map: {}", c),
                };
            }
            y += 1;
        }
        Lab { map, guard, seen: HashMap::new(), possible_obstruction_positions: HashSet::new() }
    }
}

fn main() {
    let input = include_str!("input");
    let mut lab: Lab = input.into();
    let Solution {
        distinct_visited_positions,
        possible_obstruction_positions
    } = lab.solve();
    println!("Part 1: {}", distinct_visited_positions);
    println!("Part 2: {}", possible_obstruction_positions);
}