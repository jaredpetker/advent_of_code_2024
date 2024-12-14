use itertools::Itertools;
use std::sync::LazyLock;
use regex::Regex;
use advent_of_code_2024::Vec2D;

static RE_ROBOT: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap()
);

#[derive(Debug)]
struct Robot {
    p: Vec2D,
    v: Vec2D,
}

impl<'a> From<&'a str> for Robot {
    fn from(value: &'a str) -> Self {
        let c = RE_ROBOT.captures(value).unwrap();
        let (_, [px, py, vx, vy]) = c.extract();
        Robot {
            p: Vec2D::new(px.parse().unwrap(), py.parse().unwrap()),
            v: Vec2D::new(vx.parse().unwrap(), vy.parse().unwrap()),
        }
    }
}

struct EBHQ {
    robots: Vec<Robot>,
}

impl EBHQ {
    fn tick_n(&self, t: usize, bounds: Vec2D) -> Vec<Robot> {
        self.robots.iter().map(|r| {
            Robot {
                p: Vec2D::new(
                    (r.p.x + (r.v.x * t as i64)).rem_euclid(bounds.x),
                    (r.p.y + (r.v.y * t as i64)).rem_euclid(bounds.y),
                ),
                v: r.v,
            }
        }).collect()
    }

    fn safety_factor(&self, t: usize, bounds: Vec2D) -> usize {
        let mut quadrants = vec![0, 0, 0, 0usize];
        let robots = self.tick_n(t, bounds);
        for robot in robots.iter() {
            if robot.p.x < bounds.x / 2 && robot.p.y < bounds.y / 2 {
                quadrants[0] += 1;
            } else if robot.p.x > bounds.x / 2 && robot.p.y < bounds.y / 2 {
                quadrants[1] += 1;
            } else if robot.p.x < bounds.x / 2 && robot.p.y > bounds.y / 2 {
                quadrants[2] += 1;
            } else if robot.p.x > bounds.x / 2 && robot.p.y > bounds.y / 2 {
                quadrants[3] += 1;
            }
        }
        quadrants.iter().product()
    }

    fn find_xmas_tree(&mut self, bounds: Vec2D) -> usize {
        (0..).find_or_first(|i| {
            let robots = self.tick_n(*i, bounds);
            robots.iter().map(|r| r.p).counts().values().all(|v| *v == 1)
        }).unwrap()
    }
}

impl<'a> From<&'a str> for EBHQ {
    fn from(value: &'a str) -> Self {
        EBHQ {
            robots: value.lines().map(|r| r.into()).collect()
        }
    }
}

fn main() {
    let input = include_str!("input");
    let mut ebhq = EBHQ::from(input);
    let bounds = Vec2D::new(101, 103);
    println!("Part 1: {}", ebhq.safety_factor(100, bounds));
    println!("Part 2: {}", ebhq.find_xmas_tree(bounds));
}