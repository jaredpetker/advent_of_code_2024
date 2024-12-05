use std::collections::{HashMap, HashSet};

type Page = usize;

type PageOrderingRules = HashMap<Page, HashSet<Page>>;

struct SafetyManualUpdates {
    page_ordering_rules: PageOrderingRules,
    page_updates: Vec<Vec<Page>>,
}

struct SumMiddlePages(usize, usize);

impl SafetyManualUpdates {
    fn sum_middle_page_numbers(&self) -> SumMiddlePages {
        let mut correctly_ordered_sum = 0;
        let mut incorrectly_ordered_sum = 0;
        for pages in self.page_updates.iter() {
            if self.is_correctly_ordered(pages) {
                correctly_ordered_sum += pages[pages.len() / 2];
            } else {
                let corrected = self.get_correct_order(pages);
                incorrectly_ordered_sum += corrected[corrected.len() / 2];
            }
        }
        return SumMiddlePages(correctly_ordered_sum, incorrectly_ordered_sum);
    }

    fn is_correctly_ordered(&self, pages: &Vec<Page>) -> bool {
        for i in 0..pages.len() {
            let page = pages[i];
            let before = &pages[0..i];
            let after = &pages[(i + 1)..];
            let Some(rules) = self.page_ordering_rules.get(&page) else {
                continue;
            };
            if before.iter().any(|page| rules.contains(page))
                || !after.iter().all(|page| rules.contains(page)) {
                return false;
            }
        }
        return true;
    }

    fn get_correct_order(&self, pages: &Vec<Page>) -> Vec<Page> {
        let mut corrected = pages.clone();
        corrected.sort_by(|a, b| {
            let a_rules = self.page_ordering_rules.get(a);
            let b_rules = self.page_ordering_rules.get(b);
            return if a_rules.is_some_and(|a_rules| a_rules.contains(b)) {
                std::cmp::Ordering::Less
            } else if b_rules.is_some_and(|b_rules| b_rules.contains(a)) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            };
        });
        return corrected;
    }
}

impl From<&str> for SafetyManualUpdates {
    fn from(value: &str) -> Self {
        let mut page_ordering_rules = PageOrderingRules::new();
        let mut iter = value.lines().into_iter();
        while let Some(line) = iter.next() {
            if line.is_empty() {
                break;
            }
            let mut parts = line.split("|");
            page_ordering_rules
                .entry(parts.next().unwrap().parse().unwrap())
                .or_default()
                .insert(parts.next().unwrap().parse().unwrap());
        }

        let page_updates = iter.map(
            |line| line.split(",").map(|page| page.parse().unwrap()).collect()
        ).collect();

        SafetyManualUpdates { page_ordering_rules, page_updates }
    }
}


fn main() {
    let input = include_str!("input");
    let safety_manual_updates: SafetyManualUpdates = input.into();
    let SumMiddlePages(correct, incorrect) = safety_manual_updates.sum_middle_page_numbers();
    println!("Part 1: {}", correct);
    println!("Part 2: {}", incorrect);
}