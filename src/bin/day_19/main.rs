use std::collections::HashMap;

use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Towel(Vec<u8>);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Design(Vec<u8>);

struct TowelArranger {
    map: HashMap<u8, Vec<Towel>>,
    designs: Vec<Design>,
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
enum CountingStrategy {
    Any,
    All,
}

impl TowelArranger {
    fn count_possible_designs(&self, strategy: CountingStrategy) -> usize {
        self.designs.iter().map(|Design(design)| {
            self.count_matches_for_design(design, 0, &mut HashMap::new(), strategy)
        }).sum::<usize>()
    }

    fn count_matches_for_design(&self, design: &[u8], design_idx: usize, seen: &mut HashMap<usize, usize>, strategy: CountingStrategy) -> usize {
        if design_idx == design.len() {
            return 1;
        } else if design_idx > design.len() {
            return 0;
        } else if seen.contains_key(&design_idx) {
            return seen.get(&design_idx).unwrap().clone();
        }
        let mut count = 0;
        let c = design[design_idx];
        if let Some(towels) = self.map.get(&c) {
            for Towel(towel) in towels {
                if design[design_idx..].starts_with(&towel) {
                    count += self.count_matches_for_design(design, design_idx + towel.len(), seen, strategy);
                    if strategy == CountingStrategy::Any && count > 0 {
                        break;
                    }
                }
            }
        }
        seen.insert(design_idx, count);
        return count;
    }
}

impl From<&str> for TowelArranger {
    fn from(value: &str) -> Self {
        let mut sections = value.split("\n\n");
        let towels = sections.next().unwrap().split(',').map(|s| Towel(s.trim().as_bytes().into())).collect_vec();
        let designs = sections.next().unwrap().lines().map(|s| Design(s.trim().as_bytes().into())).collect_vec();
        let mut map: HashMap<u8, Vec<Towel>> = HashMap::new();
        for towel in towels {
            let first = towel.0[0];
            map.entry(first).or_insert(vec![]).push(towel.clone());
        }
        TowelArranger { map, designs }
    }
}

fn main() {
    let input = include_str!("input");
    let towel_arranger: TowelArranger = input.into();
    println!("Part 1: {}", towel_arranger.count_possible_designs(CountingStrategy::Any));
    println!("Part 2: {}", towel_arranger.count_possible_designs(CountingStrategy::All));
}