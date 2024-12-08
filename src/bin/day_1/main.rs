use std::collections::HashMap;

type LocationId = usize;

type LocationIdList = Vec<LocationId>;

struct LocationIdLists {
    left: LocationIdList,
    right: LocationIdList,
    right_counter: HashMap<LocationId, usize>,
}

impl LocationIdLists {
    fn new() -> LocationIdLists {
        LocationIdLists {
            left: LocationIdList::new(),
            right: LocationIdList::new(),
            right_counter: HashMap::new(),
        }
    }

    fn add_locations(&mut self, location_1: LocationId, location_2: LocationId) {
        self.left.push(location_1);
        self.right.push(location_2);
        self.right_counter.entry(location_2).and_modify(|count| *count += 1).or_insert(1);
    }

    pub fn iter(&self) -> impl Iterator<Item=(&LocationId, &LocationId)> + '_ {
        self.left.iter().zip(self.right.iter())
    }

    fn find_total_distance(&self) -> usize {
        self.iter()
            .map(|(location_1, location_2)| location_1.abs_diff(*location_2))
            .sum()
    }

    fn find_similarity_score(&self) -> usize {
        self.left.iter()
            .map(|location_1| location_1 * self.right_counter.get(location_1).unwrap_or(&0))
            .sum()
    }
}

impl From<&str> for LocationIdLists {
    fn from(value: &str) -> Self {
        let mut location_id_lists = LocationIdLists::new();
        for line in value.lines() {
            let mut location_ids = line.split_whitespace();
            let location_1 = location_ids.next().unwrap().parse::<LocationId>().unwrap();
            let location_2 = location_ids.next().unwrap().parse::<LocationId>().unwrap();
            location_id_lists.add_locations(location_1, location_2);
        }
        location_id_lists.left.sort();
        location_id_lists.right.sort();
        location_id_lists
    }
}

fn main() {
    let input = include_str!("input");
    let location_id_lists = LocationIdLists::from(input);
    let total_distance = location_id_lists.find_total_distance();
    println!("Part 1: {}", &total_distance);
    let similarity_score = location_id_lists.find_similarity_score();
    println!("Part 2: {}", &similarity_score);
}