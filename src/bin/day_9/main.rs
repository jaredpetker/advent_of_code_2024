use std::fmt::Display;

enum DiskMapEntry {
    File {
        id: usize,
        size: usize,
    },
    Free {
        size: usize
    },
}

struct DiskMap {
    entries: Vec<DiskMapEntry>,
}

impl From<&str> for DiskMap {
    fn from(value: &str) -> Self {
        DiskMap {
            entries: value
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    match i % 2 {
                        0 => DiskMapEntry::File { id: i / 2, size: c.to_digit(10).unwrap() as usize },
                        _ => DiskMapEntry::Free { size: c.to_digit(10).unwrap() as usize },
                    }
                }).collect()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DiskBlock {
    File(usize),
    Free,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiskBlockGroup {
    block: DiskBlock,
    size: usize,
}

#[derive(Debug, Clone)]
struct DiskLayout {
    blocks: Vec<DiskBlock>,
    groups: Vec<DiskBlockGroup>,
    used_space: usize,
}

impl DiskLayout {
    fn compact_blocks(&mut self) {
        let mut file_tracker = self.blocks.len() - 1;
        for i in 0..self.used_space {
            if let DiskBlock::Free = self.blocks[i] {
                while DiskBlock::Free == self.blocks[file_tracker] {
                    file_tracker -= 1;
                }
                self.blocks.swap(i, file_tracker);
            }
        }
    }

    fn compact_groups(&mut self) {
        let mut last_moved_file_id = usize::MAX;
        for i in (0..self.groups.len()).rev() {
            if let DiskBlockGroup { block: DiskBlock::File(file_id), size: file_size } = self.groups[i] {
                if file_id < last_moved_file_id {
                    last_moved_file_id = file_id;
                    for j in 0..i {
                        let test_group = &mut self.groups[j];
                        if test_group.block == DiskBlock::Free && test_group.size >= file_size {
                            test_group.size -= file_size;
                            self.groups.insert(j, DiskBlockGroup { block: DiskBlock::Free, size: file_size });
                            self.groups.swap(i + 1, j);
                            break;
                        }
                    }
                }
            }
        }

        self.rebuild_blocks_from_groups();
    }

    fn rebuild_blocks_from_groups(&mut self) {
        self.blocks = self.groups.iter()
            .map(|group| std::iter::repeat(group.block.clone()).take(group.size))
            .flatten()
            .collect();
    }

    fn checksum(&self) -> usize {
        self.blocks.iter().enumerate()
            .filter_map(|(i, block)| match block {
                DiskBlock::File(id) => {
                    Some(id * i)
                }
                DiskBlock::Free => None
            }).sum()
    }
}

impl Display for DiskLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.blocks.iter() {
            write!(f, "{}", match block {
                DiskBlock::File(id) => id.to_string(),
                DiskBlock::Free => ".".to_string(),
            })?;
        }
        Ok(())
    }
}

impl From<DiskMap> for DiskLayout {
    fn from(value: DiskMap) -> Self {
        let mut used_space = 0;
        let mut blocks: Vec<DiskBlock> = Vec::new();
        let mut groups: Vec<DiskBlockGroup> = Vec::new();
        for entry in value.entries {
            match entry {
                DiskMapEntry::File { id, size } => {
                    used_space += size;
                    groups.push(DiskBlockGroup { block: DiskBlock::File(id), size });
                    blocks.extend(std::iter::repeat(DiskBlock::File(id)).take(size));
                }
                DiskMapEntry::Free { size } => {
                    groups.push(DiskBlockGroup { block: DiskBlock::Free, size });
                    blocks.extend(std::iter::repeat(DiskBlock::Free).take(size));
                }
            }
        }
        DiskLayout { blocks, groups, used_space }
    }
}

fn main() {
    let input = include_str!("input");
    let disk_map: DiskMap = input.into();
    let mut disk_layout: DiskLayout = disk_map.into();
    disk_layout.compact_blocks();
    println!("Part 1: {}", disk_layout.checksum());
    disk_layout.rebuild_blocks_from_groups();
    disk_layout.compact_groups();
    println!("Part 2: {}", disk_layout.checksum());
}