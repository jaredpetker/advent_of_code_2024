use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

type Pos = (i32, i32);

struct WordSearch {
    width: i32,
    height: i32,
    char_grid: HashMap<Pos, char>,
}

impl WordSearch {
    fn find_word_count(&self, word: &str) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                count += self.find_word_count_at_pos(word, (x, y))
            }
        }
        count
    }

    fn find_word_count_at_pos(&self, word: &str, start: Pos) -> usize {
        let mut count = 0;
        let dirs = vec![
            (0, 1), (1, 0), (0, -1), (-1, 0),
            (1, 1), (1, -1), (-1, 1), (-1, -1),
        ];
        let (start_x, start_y) = start;
        let word_bytes = word.as_bytes();
        for (dx, dy) in dirs {
            for offset in 0..word.len() {
                let x = start_x + dx * offset as i32;
                let y = start_y + dy * offset as i32;
                if self.char_grid.get(&(x, y)) != Some(&(word_bytes[offset] as char)) {
                    break;
                } else if offset == word.len() - 1 {
                    count += 1;
                }
            }
        }
        count
    }

    fn find_x_word_count(&self, word: &str) -> usize {
        if word.len() % 2 == 0 {
            return 0;
        }
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                count += self.has_x_word_at_pos(&word, (x, y)) as usize;
            }
        }
        count
    }

    fn has_x_word_at_pos(&self, word: &str, mid: Pos) -> bool {
        if self.char_grid.get(&mid) != Some(&word.chars().nth(word.len() / 2).unwrap()) {
            return false;
        }
        let dirs = vec![
            (1, 1), (-1, 1),
        ];
        let m = (word.len() / 2) as i32;
        for (dx, dy) in dirs {
            let range = -m..=m;
            let potential: String = range.filter_map(|i| {
                let x = mid.0 + dx * i;
                let y = mid.1 + dy * i;
                self.char_grid.get(&(x, y))
            }).collect();
            if !(potential.chars().eq(word.chars()) || potential.chars().rev().eq(word.chars())) {
                return false;
            }
        }
        return true;
    }
}

impl From<&str> for WordSearch {
    fn from(value: &str) -> Self {
        let mut char_grid = HashMap::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in value.lines().enumerate() {
            height = y + 1;
            for (x, letter) in line.chars().enumerate() {
                width = x + 1;
                char_grid.insert((x as i32, y as i32), letter);
            }
        }
        WordSearch { width: width as i32, height: height as i32, char_grid }
    }
}

impl Display for WordSearch {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.char_grid.get(&(x, y)).unwrap_or(&' '))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("input");
    let word_search: WordSearch = input.into();
    println!("# of XMAS: {}", word_search.find_word_count("XMAS"));
    println!("# of X-MAS: {}", word_search.find_x_word_count("MAS"));
}