use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use itertools::Itertools;
use advent_of_code_2024::Vec2D;

type Pos = Vec2D;

type Dir = Vec2D;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Obj {
    Wall,
    // points to an index in Warehouse.boxes
    BoxPosCollider(usize),
}

struct Box {
    pos: Pos,
    width: usize,
}

struct Warehouse {
    width: usize,
    height: usize,
    grid: HashMap<Pos, Obj>,
    boxes: Vec<Box>,
    moves: Vec<Dir>,
    robot: Vec2D,
}

impl Warehouse {
    fn run_move(&mut self, dir: Dir) {
        let mut queue = VecDeque::from(vec![self.robot + dir]);
        let mut seen_boxes = HashSet::new();
        let mut boxes_to_move = vec![];
        while let Some(pos) = queue.pop_front() {
            let grid_obj = self.grid.get(&pos);
            match grid_obj {
                Some(Obj::Wall) => return,
                Some(Obj::BoxPosCollider(id)) => {
                    if seen_boxes.contains(id) {
                        continue;
                    }
                    let curr_box = &self.boxes[*id];
                    for i in 0..curr_box.width {
                        let test_pos = curr_box.pos + Vec2D::new(dir.x + i as i64, dir.y);
                        queue.push_back(test_pos);
                    }
                    seen_boxes.insert(*id);
                    boxes_to_move.push(*id);
                }
                None => {}
            }
        }
        self.push_boxes(boxes_to_move, dir);
        self.robot += dir;
    }

    fn run_moves(&mut self) {
        for i in 0..self.moves.len() {
            self.run_move(self.moves[i]);
        }
    }

    fn push_boxes(&mut self, boxes: Vec<usize>, dir: Dir) {
        for id in boxes.iter().rev() {
            let mut curr_box = &mut self.boxes[*id];
            for i in 0..curr_box.width {
                let adjusted_i = if dir.x > 0 { curr_box.width - i - 1 } else { i };
                let collider_pos = curr_box.pos + Vec2D::new(adjusted_i as i64, 0);
                let next_collider_pos = collider_pos + Vec2D::new(dir.x, dir.y);
                self.grid.remove(&collider_pos);
                self.grid.insert(next_collider_pos, Obj::BoxPosCollider(*id));
            }
            curr_box.pos += dir;
        }
    }

    fn gps_coordinates(&self) -> Vec<usize> {
        self.boxes.iter().map(|b| {
            b.pos.x as usize + 100 * b.pos.y as usize
        }).collect()
    }

    fn sum_gps_coordinates(&self) -> usize {
        self.gps_coordinates().iter().sum()
    }

    fn wider_warehouse(&self, n: usize) -> Warehouse {
        let new_grid = self.grid.iter().map(|(pos, obj)| {
            vec![
                (Vec2D::new(pos.x * n as i64, pos.y), obj.clone()),
                (Vec2D::new(pos.x * n as i64 + 1, pos.y), obj.clone()),
            ]
        }).flatten().collect();
        Warehouse {
            grid: new_grid,
            moves: self.moves.clone(),
            robot: Vec2D::new(self.robot.x * n as i64, self.robot.y),
            boxes: self.boxes.iter().map(|b| Box {
                pos: Vec2D::new(b.pos.x * n as i64, b.pos.y),
                width: b.width * n,
            }).collect(),
            width: self.width * n,
            height: self.height,
        }
    }
}

impl From<&str> for Warehouse {
    fn from(value: &str) -> Self {
        let mut split = value.split("\n\n");
        let (grid_str, moves_str) = (split.next().unwrap(), split.next().unwrap());

        let mut grid = HashMap::new();
        let mut robot = Vec2D::new(0, 0);
        let mut boxes = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in grid_str.lines().enumerate() {
            height = y + 1;
            for (x, c) in line.chars().enumerate() {
                width = x + 1;
                let pos = Vec2D::new(x as i64, y as i64);
                let obj = match c {
                    '#' => Obj::Wall,
                    'O' => {
                        boxes.push(Box { pos, width: 1 });
                        Obj::BoxPosCollider(boxes.len() - 1)
                    }
                    '.' => continue,
                    '@' => {
                        robot = pos;
                        continue;
                    }
                    _ => panic!("Invalid character in input"),
                };
                grid.insert(pos, obj);
            }
        }

        let mut moves = Vec::new();
        for line in moves_str.lines() {
            moves.extend(line.chars().map(|c| match c {
                '^' => Vec2D::up(),
                '>' => Vec2D::right(),
                'v' => Vec2D::down(),
                '<' => Vec2D::left(),
                c => panic!(""),
            }));
        }

        Warehouse { grid, moves, robot, boxes, width, height }
    }
}

// Printout of the warehouse for debugging
impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid_str = String::new();

        let mut seen = HashSet::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Vec2D::new(x as i64, y as i64);
                match self.grid.get(&pos) {
                    Some(Obj::Wall) => grid_str.push('#'),
                    Some(Obj::BoxPosCollider(id)) => {
                        if seen.contains(id) {
                            continue;
                        }
                        seen.insert(id);
                        let curr_box = &self.boxes[*id];
                        grid_str.push_str(&(0..curr_box.width).map(|i| 'O').collect::<String>());
                    }
                    None => {
                        if pos == self.robot {
                            grid_str.push('@');
                        } else {
                            grid_str.push('.');
                        }
                    }
                };
            }
            grid_str.push('\n');
        }
        write!(f, "{}", grid_str)
    }
}


fn main() {
    let input = include_str!("input");
    let mut warehouse = Warehouse::from(input);
    let mut wider_warehouse = warehouse.wider_warehouse(2);
    warehouse.run_moves();
    println!("Part 1: {}", warehouse.sum_gps_coordinates());
    wider_warehouse.run_moves();
    println!("Part 2: {}", wider_warehouse.sum_gps_coordinates());
}