use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::repeat;
use std::io::{Read};
use itertools::Itertools;
use advent_of_code_2024::Vec2D;

type Pos = Vec2D;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Keypad {
    grid: HashMap<Pos, char>,
    curr: Pos,
}

impl Keypad {
    fn numeric_keypad() -> Self {
        Keypad {
            grid: HashMap::from([
                (Vec2D::new(0, 0), '7'),
                (Vec2D::new(1, 0), '8'),
                (Vec2D::new(2, 0), '9'),
                (Vec2D::new(0, 1), '4'),
                (Vec2D::new(1, 1), '5'),
                (Vec2D::new(2, 1), '6'),
                (Vec2D::new(0, 2), '1'),
                (Vec2D::new(1, 2), '2'),
                (Vec2D::new(2, 2), '3'),
                (Vec2D::new(0, 3), ' '),
                (Vec2D::new(1, 3), '0'),
                (Vec2D::new(2, 3), 'A'),
            ]),
            curr: Vec2D::new(2, 3),
        }
    }

    fn directional_keypad() -> Self {
        Keypad {
            grid: HashMap::from([
                (Vec2D::new(0, 0), ' '),
                (Vec2D::new(1, 0), '^'),
                (Vec2D::new(2, 0), 'A'),
                (Vec2D::new(0, 1), '<'),
                (Vec2D::new(1, 1), 'v'),
                (Vec2D::new(2, 1), '>'),
            ]),
            curr: Vec2D::new(2, 0),
        }
    }

    fn pathfind(&mut self, to: char) -> Vec<char> {
        // my first trial at this was a standard bfs search
        // though this failed for part 2 a bit given I had more complex
        // min path calculations with exploding combinations.
        // This is meant to be a more efficient way to get the path
        // Though I think I can take portions of this method back to the bfs...
        let to_pos = *self.grid.iter().find(|(_, &c)| c == to).unwrap().0;
        let curr = self.curr;
        let diff = to_pos - curr;
        let mut dx: Vec<char> = if diff.x > 0 {
            repeat('>').take(diff.x as usize).collect()
        } else {
            repeat('<').take(-diff.x as usize).collect()
        };
        let mut dy: Vec<char> = if diff.y > 0 {
            repeat('v').take(diff.y as usize).collect()
        } else {
            repeat('^').take(-diff.y as usize).collect()
        };
        let check_pos = Pos::new(curr.x, to_pos.y);
        let check_pos_2 = Pos::new(to_pos.x, curr.y);
        let (first, second) = if Some(&' ') == self.grid.get(&check_pos_2) {
            (dy, dx)
        } else if Some(&' ') == self.grid.get(&check_pos) {
            (dx, dy)
        } else if diff.x > 0 {
            (dy, dx)
        } else {
            (dx, dy)
        };
        self.curr = to_pos;
        first.into_iter()
            .chain(second.into_iter())
            .chain(vec!['A'].into_iter())
            .collect()
    }
}


fn main() {
    let input = include_str!("input");
    let mut keypads = vec![
        Keypad::numeric_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
    ];
    let lines = input.lines();

    let mut sum = 0;
    for line in lines {
        let mut codes = vec![line.chars().collect::<Vec<char>>()];
        let mut counter = HashMap::new();
        counter.insert(codes[0].clone(), 1);
        for keypad in &mut keypads {
            let mut next_counter = HashMap::new();

            let mut ps = vec![];
            for (code, count) in &counter {
                for &c in code {
                    let mut found_paths = keypad.pathfind(c).clone();
                    ps.push(found_paths.clone());
                    next_counter.entry(found_paths.clone())
                        .and_modify(|v| *v += count)
                        .or_insert(*count);
                }
            }

            counter = next_counter.clone();
        }
        // break;
        let v: usize = line.chars().take(3).collect::<String>().parse().unwrap();
        // println!("{} * {}", counter.iter().map(|(k, v)| k.len() * v).sum::<usize>(), v);
        sum += v * counter.iter().map(|(k, v)| k.len() * v).sum::<usize>();
    }
    println!("Sum: {}", sum);
}