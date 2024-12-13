use std::sync::LazyLock;
use regex::Regex;
use advent_of_code_2024::{Vec2D};

static RE_BUTTON: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"Button [A|B]: X\+(\d+), Y\+(\d+)").unwrap()
);

static RE_PRIZE: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap()
);

struct ButtonPresses(usize, usize);

impl ButtonPresses {
    fn tokens_required(&self) -> usize {
        self.0 * 3 + self.1
    }
}

#[derive(Debug)]
struct MachineBehavior {
    button_a: Vec2D,
    button_b: Vec2D,
    prize: Vec2D,
}

impl MachineBehavior {
    fn min_button_presses_for_prize(&self, prize_fn: Option<fn(Vec2D) -> Vec2D>) -> Option<ButtonPresses> {
        let prize = prize_fn.map_or(self.prize, |f| f(self.prize));
        let denominator = (self.button_a.x * self.button_b.y - self.button_b.x * self.button_a.y);
        let a_presses = (prize.x * self.button_b.y - self.button_b.x * prize.y) / denominator;
        let b_presses = (prize.y * self.button_a.x - self.button_a.y * prize.x) / denominator;
        if self.button_a * a_presses + self.button_b * b_presses == prize {
            Some(ButtonPresses(a_presses as usize, b_presses as usize))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Arcade(Vec<MachineBehavior>);

impl Arcade {
    fn min_tokens_required(&self, prize_fn: Option<fn(Vec2D) -> Vec2D>) -> usize {
        let Arcade(machine_behavior) = self;
        machine_behavior.iter()
            .filter_map(|machine_behavior| machine_behavior.min_button_presses_for_prize(prize_fn))
            .map(|button_presses| button_presses.tokens_required())
            .sum()
    }
}

impl From<&str> for Arcade {
    fn from(value: &str) -> Self {
        let machine_behaviors = value.split("\n\n");
        Arcade(machine_behaviors.map(|mb| mb.into()).collect())
    }
}

impl From<&str> for MachineBehavior {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        match (lines.next(), lines.next(), lines.next()) {
            (Some(a), Some(b), Some(prize)) => {
                let (_, [button_a_x, button_a_y]) = RE_BUTTON.captures(a).unwrap().extract();
                let (_, [button_b_x, button_b_y]) = RE_BUTTON.captures(b).unwrap().extract();
                let (_, [prize_x, prize_y]) = RE_PRIZE.captures(prize).unwrap().extract();
                MachineBehavior {
                    button_a: Vec2D::new(button_a_x.parse().unwrap(), button_a_y.parse().unwrap()),
                    button_b: Vec2D::new(button_b_x.parse().unwrap(), button_b_y.parse().unwrap()),
                    prize: Vec2D::new(prize_x.parse().unwrap(), prize_y.parse().unwrap()),
                }
            }
            _ => panic!("Invalid input"),
        }
    }
}

fn main() {
    let input = include_str!("input");
    let configs = Arcade::from(input);
    println!("part 1: {:?}", configs.min_tokens_required(None));
    println!("part 2: {:?}", configs.min_tokens_required(Some(|prize| {
        prize + Vec2D::new(10000000000000, 10000000000000)
    })));
}