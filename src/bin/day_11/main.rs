use itertools::Itertools;

type Stone = usize;

struct Pluto(Vec<Stone>);
struct Me;

impl Me {
    fn blink(stones: &Vec<Stone>) -> Vec<usize> {
        stones
            .iter()
            .map(|stone| {
                let stone_str = stone.to_string();
                if *stone == 0 {
                    vec![1]
                } else if stone_str.len() % 2 == 0 {
                    let split = stone_str.split_at(stone_str.len() / 2);
                    vec![split.0.parse().unwrap(), split.1.parse().unwrap()]
                } else {
                    vec![*stone * 2024]
                }
            })
            .flatten()
            .collect()
    }


    fn blink_n(stones: &Vec<Stone>, times: usize) -> usize {
        // this is a silly way to do this, but wanted to have some fun with it
        (0..times)
            .fold(stones.into_iter().cloned().counts(), |histogram, _| {
                histogram.iter().map(|(stone, count)| {
                    Me::blink(&vec![*stone])
                        .iter()
                        .map(|blinked_stone| (*blinked_stone, *count)).collect_vec()
                }).flatten().into_grouping_map().sum()
            }).values().sum()
    }
}

impl From<&str> for Pluto {
    fn from(value: &str) -> Self {
        Pluto(value.split_whitespace().map(|s| s.parse().unwrap()).collect())
    }
}

fn main() {
    let input = include_str!("input");
    let Pluto(stones) = input.into();
    println!("Part 1: {}", Me::blink_n(&stones, 25));
    println!("Part 2: {}", Me::blink_n(&stones, 75));
}