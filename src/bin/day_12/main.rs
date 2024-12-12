use std::collections::{HashMap, HashSet};
use advent_of_code_2024::Vec2D;

#[derive(Debug)]
struct Garden {
    plot: HashMap<Vec2D, char>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct Region {
    plot_id: char,
    plots: HashSet<Vec2D>,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        self.plots.iter().map(|plot| !self.plots.contains(&plot.add(&Vec2D::up())) as usize
            + !self.plots.contains(&plot.add(&Vec2D::down())) as usize
            + !self.plots.contains(&plot.add(&Vec2D::left())) as usize
            + !self.plots.contains(&plot.add(&Vec2D::right())) as usize).sum()
    }

    fn sides(&self) -> usize {
        let modifiers = &[Vec2D::new(1, 1), Vec2D::new(1, -1), Vec2D::new(-1, -1), Vec2D::new(-1, 1)];
        self.plots.iter().map(|plot| {
            modifiers.iter().map(|modifier| {
                (((self.plots.contains(&(*plot + Vec2D::right().prod_vec(&modifier))) &&
                    self.plots.contains(&(*plot + Vec2D::up().prod_vec(&modifier)))) &&
                    !self.plots.contains(&(*plot + (Vec2D::right() + Vec2D::up()).prod_vec(&modifier))))
                    ||
                    (!self.plots.contains(&(*plot + Vec2D::right().prod_vec(&modifier))) &&
                        !self.plots.contains(&(*plot + Vec2D::up().prod_vec(&modifier)))))
                    as usize
            }).sum::<usize>()
        }).sum()
    }

}

impl Garden {
    fn find_regions(&self) -> Vec<Region> {
        let mut visited = HashMap::new();
        let mut regions: Vec<Region> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Vec2D::new(x as i64, y as i64);
                if visited.contains_key(&pos) {
                    continue;
                }
                let mut queue = vec![pos];
                let plot_id = *self.plot.get(&pos).unwrap();
                let mut region = Region { plot_id, plots: HashSet::new() };
                while let Some(current) = queue.pop() {
                    if visited.contains_key(&current) {
                        continue;
                    }
                    let Some(curr) = self.plot.get(&current) else {
                        continue;
                    };
                    if region.plot_id == *curr {
                        region.plots.insert(current);
                        visited.insert(current, true);
                        queue.push(current.add(&Vec2D::up()));
                        queue.push(current.add(&Vec2D::down()));
                        queue.push(current.add(&Vec2D::left()));
                        queue.push(current.add(&Vec2D::right()));
                    }
                }
                regions.push(region);
            }
        }
        regions
    }

    fn fence_price_by_perimeter(regions: &Vec<Region>) -> usize {
        regions.iter().map(|region| region.area() * region.perimeter()).sum()
    }

    fn fence_price_by_sides(regions: &Vec<Region>) -> usize {
        regions.iter().map(|region| region.area() * region.sides()).sum()
    }
}

impl From<&str> for Garden {
    fn from(value: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut plot = HashMap::new();
        for (y, line) in value.lines().enumerate() {
            height = y + 1;
            for (x, c) in line.chars().enumerate() {
                width = x + 1;
                plot.insert(Vec2D::new(x as i64, y as i64), c);
            }
        }
        Garden { plot, width, height }
    }
}

fn main() {
    let input = include_str!("input");
    let garden = Garden::from(input);
    let regions = garden.find_regions();
    println!("Part 1: {:?}", Garden::fence_price_by_perimeter(&regions));
    println!("Part 2: {:?}", Garden::fence_price_by_sides(&regions));
}