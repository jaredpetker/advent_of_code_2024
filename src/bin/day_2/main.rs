use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
enum LevelDirection {
    Increasing,
    Decreasing,
    Unknown,
}
type Level = usize;
struct Report(Vec<Level>);

impl Report {
    fn is_safe(&self, adjacent_levels_tolerance_range: Range<Level>, skip_level_index: Option<usize>) -> bool {
        let mut level_direction = LevelDirection::Unknown;
        for (i, level) in self.0.iter().enumerate().skip(1) {
            let mut last_level = self.0[i - 1];
            if let Some(skip_level_index) = skip_level_index {
                if i == skip_level_index {
                    continue;
                } else if i - 1 == skip_level_index {
                    if skip_level_index > 0 {
                        last_level = self.0[i - 2];
                    } else {
                        continue;
                    }
                }
            }
            let next_level_direction = if *level > last_level {
                LevelDirection::Increasing
            } else {
                LevelDirection::Decreasing
            };
            let in_range = adjacent_levels_tolerance_range.contains(&last_level.abs_diff(*level));
            if (level_direction != LevelDirection::Unknown && level_direction != next_level_direction) || !in_range {
                return false;
            }
            level_direction = next_level_direction;
        }
        return true;
    }


    fn is_safe_with_bad_level(&self, adjacent_levels_tolerance_range: Range<Level>) -> bool {
        let mut range_to_check = 0..self.0.len();
        range_to_check
            .any(|i| self.is_safe(adjacent_levels_tolerance_range.clone(), Some(i)))
    }
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        Report(value.split_whitespace().map(|level| level.parse::<Level>().unwrap()).collect())
    }
}

struct Reports(Vec<Report>);

impl Reports {
    fn count_safe_reports(&self, adjacent_levels_tolerance_range: Range<Level>, allow_bad_level: bool) -> usize {
        if !allow_bad_level {
            self.0.iter().filter(|report| report.is_safe(adjacent_levels_tolerance_range.clone(), None)).count()
        } else {
            self.0.iter().filter(|&Report(levels)| {
                let mut range = 0..levels.len();
                range.any(|i|{
                    let split = levels.split_at(i);
                    let mut check_vec = Vec::new();
                    check_vec.extend(split.0);
                    check_vec.extend(split.1);
                    Report(check_vec).is_safe_with_bad_level(adjacent_levels_tolerance_range.clone())
                })
            }).count()
        }
    }
}

impl From<&str> for Reports {
    fn from(value: &str) -> Self {
        Reports(value.lines().map(|line| Report::from(line)).collect())
    }
}

fn main() {
    let input = include_str!("input");
    let reports = Reports::from(input);
    let num_safe_reports = reports.count_safe_reports(1..4, false);
    println!("Number of safe reports: {}", num_safe_reports);
    let num_safe_reports_allowing_bad_level = reports.count_safe_reports(1..4, true);
    println!("Number of safe reports while allowing bad level: {}", num_safe_reports_allowing_bad_level);
}