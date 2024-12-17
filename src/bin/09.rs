use std::{collections::VecDeque, fmt::Display};

advent_of_code_2024::solution!(9);

struct DiskMap {
    disk: VecDeque<Block>,
}

#[derive(Copy, Clone, Debug)]
enum Block {
    File { id: usize, size: usize },
    Free { size: usize },
}

impl DiskMap {
    fn decompress(self) -> Self {
        Self {
            disk: self
                .disk
                .into_iter()
                .flat_map(|block| match block {
                    Block::File { id, size } => {
                        vec![Block::File { id, size: 1 }; size]
                    }
                    Block::Free { size } => {
                        vec![Block::Free { size: 1 }; size]
                    }
                })
                .collect::<VecDeque<Block>>(),
        }
    }

    fn defragment(&self) -> Self {
        let mut res = VecDeque::new();

        let mut left_ptr = 0;
        let mut right_ptr = self.disk.len();

        loop {
            let left = self.disk.get(left_ptr).expect("left should be valid");
            if let Block::File { .. } = left {
                res.push_back(*left);
                left_ptr += 1
            }

            let right = self.disk.get(right_ptr - 1).expect("right should be valid");
            if let Block::Free { .. } = right {
                right_ptr -= 1
            }

            if left_ptr == right_ptr {
                break;
            }

            if let (Block::Free { .. }, Block::File { .. }) = (left, right) {
                res.push_back(*right);
                left_ptr += 1;
                right_ptr -= 1;
            }
        }

        Self { disk: res }
    }

    fn defragment_whole_blocks(&mut self) -> Self {
        let mut disk = VecDeque::new();

        while let Some(block) = self.disk.pop_front() {
            match block {
                Block::File { .. } => disk.push_back(block),
                Block::Free { size: mut free } => {
                    (0..self.disk.len()).rev().for_each(|i| {
                        if let Block::File { size, .. } = self.disk[i] {
                            if size <= free {
                                disk.push_back(self.disk[i]);
                                self.disk.remove(i);
                                self.disk.insert(i, Block::Free { size });
                                free -= size;
                            }
                        }
                    });

                    if free > 0 {
                        disk.push_back(Block::Free { size: free });
                    }
                }
            }
        }

        Self { disk }
    }

    fn sum(&self) -> usize {
        self.disk
            .iter()
            .enumerate()
            .fold(0, |acc, (index, block)| match block {
                Block::File { id, .. } => acc + (id * index),
                _ => acc,
            })
    }
}

impl From<&str> for DiskMap {
    fn from(value: &str) -> Self {
        let disk: VecDeque<Block> =
            value
                .chars()
                .enumerate()
                .fold(VecDeque::new(), |mut block, (index, char)| {
                    if let Some(size) = char.to_digit(10) {
                        match index % 2 {
                            0 => block.push_back(Block::File {
                                id: index / 2,
                                size: size as usize,
                            }),
                            _ => block.push_back(Block::Free {
                                size: size as usize,
                            }),
                        }
                    }

                    block
                });

        Self { disk }
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        for block in &self.disk {
            match block {
                Block::File { id, size } => {
                    for _ in 0..*size {
                        write!(f, "{}", id)?
                    }
                }
                Block::Free { size } => {
                    for _ in 0..*size {
                        write!(f, ".")?
                    }
                }
            }
        }

        writeln!(f)?;

        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let disk_map = DiskMap::from(input);

    Some(disk_map.decompress().defragment().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut disk_map = DiskMap::from(input);

    Some(disk_map.defragment_whole_blocks().decompress().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_example;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(2858));
    }
}
