use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
enum LevelDirection {
    Increasing,
    Decreasing,
    Unknown,
}
type Level = usize;
struct Report {
    levels: Vec<Level>,
}

impl Report {
    fn is_safe(&self, adjacent_levels_tolerance_range: Range<Level>, allow_bad_level: bool) -> bool {
        let mut level_direction = LevelDirection::Unknown;
        let mut bad_level_found = false;
        for (i, level) in self.levels.iter().enumerate().skip(1) {
            let mut last_level = self.levels[i - 1];
            let next_level_direction = if *level > last_level {
                LevelDirection::Increasing
            } else {
                LevelDirection::Decreasing
            };
            let in_range = adjacent_levels_tolerance_range.contains(&last_level.abs_diff(*level));
            if (level_direction != LevelDirection::Unknown && level_direction != next_level_direction) || !in_range {
                if allow_bad_level && !bad_level_found {
                    bad_level_found = true;
                    continue;
                } else {
                    return false;
                }
            }
            level_direction = next_level_direction;
        }
        return true;
    }
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        Report {
            levels: value.split_whitespace().map(|level| level.parse::<Level>().unwrap()).collect()
        }
    }
}

struct Reports {
    reports: Vec<Report>,
}

impl Reports {
    fn count_safe_reports(&self, adjacent_levels_tolerance_range: Range<Level>, allow_bad_level: bool) -> usize {
        self.reports.iter().filter(|report| report.is_safe(adjacent_levels_tolerance_range.clone(), allow_bad_level)).count()
    }
}

impl From<&str> for Reports {
    fn from(value: &str) -> Self {
        Reports {
            reports: value.lines().map(|line| Report::from(line)).collect()
        }
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