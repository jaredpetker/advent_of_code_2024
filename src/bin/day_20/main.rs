use std::collections::{HashMap};
use std::ops::{RangeBounds};
use advent_of_code_2024::Vec2D;

type Pos = Vec2D;

struct OrderedTrack(Vec<(Pos, usize)>);

impl OrderedTrack {
    // This method is better for part 2 than it is for part 1 wrt efficiency, but hey, works for both :)
    fn count_cheats<R: RangeBounds<usize>>(&self, skip_range: R, savings_threshold: usize) -> usize {
        let mut count = 0;
        for (i, (pos, distance)) in self.0.iter().enumerate() {
            for j in i + 1..self.0.len() {
                let (pos_2, distance_2) = self.0[j];
                let dx = pos.x.abs_diff(pos_2.x);
                let dy = pos.y.abs_diff(pos_2.y);
                let skip_length = (dx + dy) as usize;
                if !skip_range.contains(&skip_length) {
                    continue;
                }
                let cheat_savings = distance.saturating_sub(distance_2).saturating_sub(skip_length);
                count += (cheat_savings >= savings_threshold) as usize;
            }
        }
        count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum MapItem {
    Wall,
    Track(usize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RaceTrack {
    map: HashMap<Pos, MapItem>,
    start: Pos,
    end: Pos,
}

impl RaceTrack {
    fn get_ordered_track(&mut self) -> OrderedTrack {
        let mut queue = vec![(self.end, 0)];
        let mut track: Vec<(Pos, usize)> = vec![];
        while let Some((pos, distance)) = queue.pop() {
            if let Some(MapItem::Track(d)) = self.map.get_mut(&pos) {
                if *d < usize::MAX {
                    continue;
                }
                self.map.insert(pos, MapItem::Track(distance));
                track.push((pos, distance));
                if pos == self.start {
                    break;
                }
                for dir in [Vec2D::up(), Vec2D::down(), Vec2D::left(), Vec2D::right()] {
                    queue.push((pos + dir, distance + 1));
                }
            }
        }
        track.reverse();
        return OrderedTrack(track);
    }

}

impl From<&str> for RaceTrack {
    fn from(value: &str) -> Self {
        let mut map = HashMap::new();
        let mut start = Pos::default();
        let mut end = Pos::default();
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Pos::new(x as i64, y as i64);
                match c {
                    '#' => {
                        map.insert(pos, MapItem::Wall);
                    }
                    '.' => {
                        map.insert(pos, MapItem::Track(usize::MAX));
                    }
                    'S' => {
                        start = pos;
                        map.insert(pos, MapItem::Track(usize::MAX));
                    }
                    'E' => {
                        end = pos;
                        map.insert(pos, MapItem::Track(usize::MAX));
                    }
                    _ => panic!("Invalid character in input")
                }
            }
        }
        Self {
            map,
            start,
            end,
        }
    }
}

fn main() {
    let input = include_str!("input");
    let mut track: RaceTrack = input.into();
    let ordered_track = track.get_ordered_track();
    println!("Part 1: {}", ordered_track.count_cheats(2..=2, 100));
    println!("Part 2: {}", ordered_track.count_cheats(2..=20, 100));
}