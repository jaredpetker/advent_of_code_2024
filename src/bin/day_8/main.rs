use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter};
use itertools::Itertools;
use advent_of_code_2024::{Rect2D, Vec2D};

#[derive(Debug, Clone, Default)]
struct City {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<Vec2D>>,
}

struct Antinodes {
    resonant: usize,
    non_resonant: usize
}

impl City {
    fn compute_antinodes(&self) -> Antinodes {
        let bounds = Rect2D::new(Vec2D::default(), Vec2D::new(self.width as i64, self.height as i64));
        let mut antinodes: HashSet<Vec2D> = HashSet::new();
        let mut antinodes_with_resonant_harmonics: HashSet<Vec2D> = HashSet::from_iter(
            self.antennas.values().cloned().flatten().into_iter()
        );

        self.antennas
            .values()
            .map(|antenna| antenna.iter().combinations(2))
            .flatten()
            .for_each(|pair| {
                for (a, b) in [(pair[0], pair[1]), (pair[1], pair[0])] {
                    let diff = Vec2D::new(a.x - b.x, a.y - b.y);
                    let mut antinode = *a + diff;
                    let first_antinode = antinode.clone();
                    while bounds.contains(antinode) {
                        if antinode == first_antinode {
                            antinodes.insert(antinode);
                        }
                        antinodes_with_resonant_harmonics.insert(antinode);
                        antinode += diff;
                    }
                }
            });


        Antinodes {
            non_resonant: antinodes.len(),
            resonant: antinodes_with_resonant_harmonics.len()
        }
    }
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut grid = vec![vec!['.'; self.width as usize]; self.height as usize];
        for (antenna, positions) in self.antennas.iter() {
            for position in positions {
                grid[position.y as usize][position.x as usize] = *antenna;
            }
        }
        for row in grid {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl From<&str> for City {
    fn from(value: &str) -> Self {
        let mut city = City::default();
        for (y, line) in value.lines().enumerate() {
            city.height = y + 1;
            for (x, c) in line.chars().enumerate() {
                city.width = x + 1;
                if c != '.' {
                    city.antennas.entry(c).or_default().push(Vec2D::new(x as i64, y as i64));
                }
            }
        }
        city
    }
}


fn main() {
    let input = include_str!("input");
    let city: City = input.into();
    let antinodes = city.compute_antinodes();
    println!("Part 1: {}", antinodes.non_resonant);
    println!("Part 2: {}", antinodes.resonant);
}