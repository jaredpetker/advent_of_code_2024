use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};

use itertools::Itertools;

use advent_of_code_2024::Vec2D;

type Pos = Vec2D;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Coord {
    Free,
    Corrupted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MemorySpace {
    width: usize,
    height: usize,
    start: Pos,
    end: Pos,
    grid: HashMap<Pos, Coord>,
}

#[derive(Clone, Debug)]
struct Item {
    w: usize,
    v: Pos
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.w.cmp(&other.w)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w && self.v == other.v
    }
}

impl Eq for Item {}


impl MemorySpace {
    fn drop_bytes(&mut self, bytes: &Vec<Pos>, n: usize) {
        self.grid.extend(bytes.iter().take(n).map(|pos| (pos.clone(), Coord::Corrupted)));
    }

    fn is_grid_pos(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width as i64 && pos.y < self.height as i64
    }

    fn get_steps_til_end(&self) -> usize {
        let mut heap = BinaryHeap::new();
        let mut dist: HashMap<Pos, usize> = HashMap::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos::new(x as i64, y as i64);
                if self.grid.get(&pos) != Some(&Coord::Corrupted) {
                    dist.insert(pos.clone(), usize::MAX);
                }
            }
        }
        dist.insert(self.start.clone(), 0);
        heap.push(Reverse(Item { w: 0, v: self.start.clone() }));
        while let Some(Reverse(Item { w: _, v })) = heap.pop() {
            for dir in [Pos::up(), Pos::left(), Pos::right(), Pos::down()].iter().cloned() {
                let next_pos = v.clone() + dir;
                if self.is_grid_pos(&next_pos) && self.grid.get(&next_pos) != Some(&Coord::Corrupted) {
                    // println!("{:?}", dist);
                    if *dist.get(&next_pos).unwrap() > dist.get(&v).unwrap() + 1 {
                        dist.insert(next_pos.clone(), dist.get(&v).unwrap() + 1);
                        heap.push(Reverse(Item { w: dist.get(&next_pos).unwrap().clone(), v: next_pos }));
                    }
                }
            }
        }

        return *dist.get(&self.end).unwrap()
    }

    fn find_blocking_byte(&self, bytes_to_drop: &Vec<Pos>) -> Pos {
        let mut lo =  0;
        let mut hi = bytes_to_drop.len();
        let mut mid = lo + (hi - lo) / 2;

        while lo < hi {
            mid = lo + (hi - lo) / 2;
            let mut cloned = self.clone();
            cloned.drop_bytes(&bytes_to_drop, mid);
            let x = cloned.get_steps_til_end();
            if x == usize::MAX {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        bytes_to_drop[mid]
    }
}

impl Display for MemorySpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos::new(x as i64, y as i64);
                let c = match self.grid.get(&pos) {
                    Some(Coord::Free) => '.',
                    Some(Coord::Corrupted) => '#',
                    None => ' ',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


fn main() {
    let input = include_str!("input");
    let bytes_to_drop = input.lines()
        .map(|line| line.split(","))
        .map(|mut split| Pos::new(split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap()))
        .collect_vec();
    let size = 70;

    let mut memory_space = MemorySpace {
        width: size + 1,
        height: size + 1,
        start: Pos::new(0, 0),
        end: Pos::new(size as i64, size as i64),
        grid: HashMap::new(),
    };
    let cloned = memory_space.clone();
    memory_space.drop_bytes(&bytes_to_drop, 1024);
    println!("Part 1: {}", memory_space.get_steps_til_end());
    let blocking_byte = cloned.find_blocking_byte(&bytes_to_drop);
    println!("Part 2: {},{}", blocking_byte.x, blocking_byte.y);

}