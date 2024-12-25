use std::collections::{HashMap, HashSet};
use itertools::Itertools;

type Computer = String;

type Triple = (Computer, Computer, Computer);

#[derive(Clone, Debug, Eq, PartialEq)]
struct LocalNetwork {
    network_map: HashMap<Computer, HashSet<Computer>>,
}

impl LocalNetwork {
    fn count_triples(&self) -> usize {
        let mut seen: HashSet<Computer> = HashSet::new();
        let mut triples = vec![];
        let computers_starting_with_t = self.network_map.iter().filter(|(k, _)| k.starts_with("t"));
        for (computer, connections) in computers_starting_with_t {
            seen.insert(computer.clone());
            for window in connections.iter().combinations(2) {
                let (first, second) = (window[0], window[1]);
                if seen.contains(first) || seen.contains(second) {
                    continue;
                }
                if self.network_map.get(first).unwrap().contains(second) {
                    triples.push((computer.clone(), first.clone(), second.clone()));
                }
            }
        }
        return triples.len();
    }

    fn find_nlets(&self) -> String {
        let mut seen: HashSet<Computer> = HashSet::new();
        let mut max_connections = vec![];
        for (computer, connections) in self.network_map.iter() {
            seen.insert(computer.clone());
            for n in (0..connections.len()).rev() {
                if n < max_connections.len() {
                    break;
                }
                'combo: for combo in connections.iter().combinations(n) {
                    for pair in combo.iter().combinations(2) {
                        let (&first, &second) = (pair[0], pair[1]);
                        if !self.network_map.get(first).unwrap().contains(second) {
                            break 'combo;
                        }
                    }
                    max_connections.clear();
                    max_connections.push(computer);
                    max_connections.extend(combo.iter().cloned());
                    max_connections.sort();
                }
            }
        }
        max_connections.into_iter().join(",")
    }
}

impl From<&str> for LocalNetwork {
    fn from(value: &str) -> Self {
        let mut network_map = HashMap::new();
        for line in value.lines() {
            let mut parts = line.split("-");
            let first = parts.next().unwrap().to_string();
            let second = parts.next().unwrap().to_string();
            network_map.entry(first.clone()).or_insert(HashSet::new()).insert(second.clone());
            network_map.entry(second.clone()).or_insert(HashSet::new()).insert(first.clone());
        }
        LocalNetwork { network_map }
    }
}

fn main() {
    let input = include_str!("input");
    let network = LocalNetwork::from(input);
    println!("Part 1: {}", network.count_triples());
    println!("Part 2: {}", network.find_nlets());
}