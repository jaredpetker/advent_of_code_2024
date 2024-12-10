use std::collections::HashMap;
use itertools::Itertools;
use advent_of_code_2024::Vec2D;

type Pos = Vec2D;

struct State {
    pos: Pos,
    height: usize,
}

#[derive(Debug, Clone, Default)]
struct FloatingIsland {
    topo_map: HashMap<Pos, usize>,
    trailheads: Vec<Pos>,
}

struct TrailMetrics {
    score: usize,
    rating: usize,
}

impl FloatingIsland {
    fn get_trail_metrics(&self) -> TrailMetrics {
        let peaks_by_trailhead = self.traversable_peaks_by_trailhead();
        TrailMetrics {
            score: peaks_by_trailhead.values().map(|v| v.iter().unique()).flatten().count(),
            rating: peaks_by_trailhead.values().flatten().count(),
        }
    }

    fn traversable_peaks_by_trailhead(&self) -> HashMap<Pos, Vec<Pos>> {
        HashMap::from_iter(self.trailheads.iter()
            .map(|trailhead| {
                (trailhead.clone(), self.traversable_peaks_for_trailhead(trailhead.clone()))
            })
        )
    }

    fn traversable_peaks_for_trailhead(&self, trailhead: Pos) -> Vec<Pos> {
        let mut peaks = vec![];
        let peak_height = 9usize;
        let max_step_height = 1usize;

        let mut queue = vec![State { pos: trailhead, height: 0 }];
        while let Some(current) = queue.pop() {
            for dir in [Vec2D::up(), Vec2D::down(), Vec2D::left(), Vec2D::right()] {
                let next_pos = current.pos + dir;
                if self.topo_map.get(&next_pos).is_none() {
                    continue;
                }
                let next_height = *self.topo_map.get(&next_pos).unwrap();
                if next_height == current.height + max_step_height {
                    if next_height == peak_height {
                        peaks.push(next_pos);
                    } else {
                        queue.push(State { pos: next_pos, height: *next_height });
                    }
                }
            }
        }

        peaks
    }
}

impl From<&str> for FloatingIsland {
    fn from(value: &str) -> Self {
        let mut floating_island = FloatingIsland::default();
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let h = c.to_digit(10).unwrap();
                floating_island.topo_map.insert(Pos::new(x as i64, y as i64), h as usize);
                if h == 0 {
                    floating_island.trailheads.push(Pos::new(x as i64, y as i64));
                }
            }
        }
        floating_island
    }
}

fn main() {
    let input = include_str!("input");
    let floating_island: FloatingIsland = input.into();
    let TrailMetrics { score, rating } = floating_island.get_trail_metrics();
    println!("Part 1: {}", score);
    println!("Part 2: {}", rating);
}
