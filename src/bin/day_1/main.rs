use std::collections::HashMap;

type LocationId = i32;

struct LocationIdList {
    location_ids: Vec<LocationId>,
}

impl LocationIdList {
    fn new() -> LocationIdList {
        LocationIdList {
            location_ids: Vec::new(),
        }
    }

    fn add_location(&mut self, location_id: LocationId) {
        let mut insertion_index = self.location_ids.len();
        for (index, &current_location_id) in self.location_ids.iter().enumerate() {
            if location_id < current_location_id {
                insertion_index = index;
                break;
            }
        }
        self.location_ids.insert(insertion_index, location_id);
    }

    fn get_locations(&self) -> &Vec<LocationId> {
        &self.location_ids
    }
}

struct LocationIdLists {
    left: LocationIdList,
    right: LocationIdList,
    right_counter: HashMap<i32, i32>,
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
        self.left.add_location(location_1);
        self.right.add_location(location_2);
        self.right_counter.entry(location_2).and_modify(|count| *count += 1).or_insert(1);
    }

    fn find_total_distance(&self) -> i32 {
        let mut total_distance = 0;
        for (location_1, location_2) in self.left.get_locations().iter().zip(self.right.get_locations().iter()) {
            total_distance += (location_2 - location_1).abs();
        }
        total_distance
    }

    fn find_similarity_score(&self) -> i32 {
        let mut similarity_score = 0;
        for location_1 in self.left.get_locations().iter() {
            let right_counter = self.right_counter.get(location_1).unwrap_or(&0);
            similarity_score += location_1 * right_counter;
        }
        similarity_score
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
        location_id_lists
    }
}

fn main() {
    let input = include_str!("input");
    let location_id_lists = LocationIdLists::from(input);
    let total_distance = location_id_lists.find_total_distance();
    println!("{}", &total_distance);
    let similarity_score = location_id_lists.find_similarity_score();
    println!("{}", &similarity_score);
}