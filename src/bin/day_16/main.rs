use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;

use advent_of_code_2024::Vec2D;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Reindeer {
    pos: Vec2D,
    dir: Vec2D,
    score: usize,
    seen: HashSet<(Vec2D, Vec2D)>,
}

type Pos = Vec2D;
type Dir = Vec2D;

#[derive(Clone, Debug, Eq, PartialEq)]
enum MazeTile {
    Wall,
    Open,
}

struct ReindeerMaze {
    map: HashMap<Pos, MazeTile>,
    start: Vec2D,
    end: Vec2D,
}

struct Pathfind {
    lowest_score: usize,
    n_best_sitting_spots: usize,
}

impl ReindeerMaze {
    fn pathfind(&mut self) -> Pathfind {
        let reindeer = Reindeer {
            pos: self.start,
            dir: Dir::right(),
            score: 0,
            seen: HashSet::from([(self.start, Dir::right())]),
        };

        let mut queue = VecDeque::from(vec![reindeer]);
        let mut move_to_path_map: HashMap<(Pos, Dir), HashSet<(Pos, Dir)>> = HashMap::new();
        let mut move_to_score_map: HashMap<(Pos, Dir), usize> = HashMap::new();
        let mut curr_lowest_score = usize::MAX;
        let mut curr_lowest_path = HashSet::new();

        while let Some(r) = queue.pop_front() {
            for dir in [Dir::right(), Dir::up(), Dir::left(), Dir::down()].iter().cloned() {
                let curr_pos = r.pos.clone() + dir;
                let curr_score = r.score + self.calculate_move_score(&r, dir);

                if curr_score > curr_lowest_score {
                    continue;
                }

                let mut cloned_reindeer = r.clone();
                cloned_reindeer.score = curr_score;
                cloned_reindeer.pos = curr_pos;
                cloned_reindeer.dir = dir;
                cloned_reindeer.seen = r.seen.clone();
                cloned_reindeer.seen.insert((curr_pos.clone(), dir));

                if let Some(move_score) = move_to_score_map.get(&(curr_pos, dir)) {
                    if *move_score < curr_score {
                        continue;
                    } else if *move_score == curr_score {
                        move_to_path_map.entry((curr_pos, dir))
                            .and_modify(|x| x.extend(cloned_reindeer.seen.iter()));
                        continue;
                    }
                }

                move_to_path_map.entry((curr_pos, dir))
                    .and_modify(|path| {
                        path.clear();
                        path.extend(cloned_reindeer.seen.iter());
                    })
                    .or_default()
                    .extend(cloned_reindeer.seen.iter());

                move_to_score_map.insert((curr_pos, dir), curr_score);

                if curr_pos == self.end {
                    if curr_score< curr_lowest_score {
                        curr_lowest_path = cloned_reindeer.seen.clone();
                        curr_lowest_score = curr_score
                    }
                } else if self.map.get(&curr_pos) == Some(&MazeTile::Open) {
                    queue.push_back(cloned_reindeer);
                } else {
                    continue;
                }
            }
        }

        let mut lowest_paths: HashSet<Vec2D> = HashSet::from_iter(
            curr_lowest_path.iter().map(|(p, _)| *p)
        );

        for i in curr_lowest_path.clone().iter() {
            if let Some(path) = move_to_path_map.get(&i) {
                lowest_paths.extend(path.iter().map(|(p, _)| p));
            }
        }

        Pathfind {
            lowest_score: curr_lowest_score,
            n_best_sitting_spots: lowest_paths.len(),
        }
    }

    fn calculate_move_score(&self, r: &Reindeer, dir: Vec2D) -> usize {
        if r.dir == dir {
            1
        } else if r.dir == dir * -1 {
            2000 + 1
        } else {
            1000 + 1
        }
    }
}

impl From<&str> for ReindeerMaze {
    fn from(value: &str) -> Self {
        let mut grid = HashMap::new();
        let mut start = Vec2D::new(0, 0);
        let mut end = Vec2D::new(0, 0);
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Vec2D::new(x as i64, y as i64);
                match c {
                    '#' => {
                        grid.insert(pos, MazeTile::Wall);
                    }
                    '.' => {
                        grid.insert(pos, MazeTile::Open);
                    }
                    'S' => {
                        grid.insert(pos, MazeTile::Open);
                        start = pos;
                    }
                    'E' => {
                        grid.insert(pos, MazeTile::Open);
                        end = pos;
                    }
                    _ => {}
                }
            }
        }
        ReindeerMaze { map: grid, start, end }
    }
}

fn main() {
    let input = include_str!("input");
    let mut maze = ReindeerMaze::from(input);
    let Pathfind {
        lowest_score,
        n_best_sitting_spots
    } = maze.pathfind();
    println!("Part 1: {}", lowest_score);
    println!("Part 2: {}", n_best_sitting_spots);
}