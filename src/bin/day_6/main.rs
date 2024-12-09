use std::collections::{HashMap, HashSet};
use advent_of_code_2024::Vec2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Path {
    Clear,
    Obstacle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Valid,
    Invalid
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

    fn new() -> Self {
        Lab {
            map: HashMap::new(),
            guard: Guard { pos: Vec2D::default(), dir: Vec2D::up() },
            seen: HashMap::new(),
            possible_obstruction_positions: HashSet::new(),
        }
    }

    fn set_path(&mut self, pos: Vec2D, path: Path) {
        self.map.insert(pos, path);
    }

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
            _ => Step::Invalid
        }
    }

    fn has_loop(&mut self) -> bool {
        loop {
            self.mark_guard_pos_as_seen();
            match self.step() {
                Step::Valid => {
                    if self.seen.get(&self.guard.pos).is_some_and(|seen| seen.contains(&self.guard.dir)) {
                        return true
                    }
                }
                _ => return false
            }
        }
    }

    fn mark_guard_pos_as_seen(&mut self) {
        self.seen.entry(self.guard.pos.clone()).or_default().insert(self.guard.dir.clone());
    }

    fn solve(&mut self) -> Solution {
        loop {
            let next_pos = self.guard.next_pos();
            if Some(&Path::Clear) == self.map.get(&next_pos) && !self.seen.contains_key(&next_pos) {
                let mut cloned_lab = self.clone();
                cloned_lab.set_path(next_pos, Path::Obstacle);
                if cloned_lab.has_loop() {
                    self.possible_obstruction_positions.insert(next_pos);
                }
            }

            self.mark_guard_pos_as_seen();
            if self.step() == Step::Invalid {
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
        let mut lab = Lab::new();
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