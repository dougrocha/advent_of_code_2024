use std::collections::HashMap;

advent_of_code_2024::solution!(11);

struct Stones {
    stones: HashMap<u64, usize>,
}

impl From<&str> for Stones {
    fn from(value: &str) -> Self {
        Self {
            stones: value
                .split_whitespace()
                .filter_map(|x| x.parse::<u64>().ok())
                .fold(HashMap::new(), |mut acc, stone| {
                    *acc.entry(stone).or_insert(0) += 1;
                    acc
                }),
        }
    }
}

impl Stones {
    fn blink(&mut self, blinks: usize) -> u64 {
        (0..blinks).for_each(|_| {
            self.stones
                .clone()
                .iter()
                .filter(|(_, &x)| x != 0)
                .for_each(|(&stone, &value)| {
                    if stone == 0 {
                        self.insert_cache(1, value);
                        self.remove_cache(stone, value);
                    } else if stone.to_string().len() % 2 == 0 {
                        let str = stone.to_string();
                        let (left, right) = str.split_at(str.len() / 2);

                        self.insert_cache(right.parse::<u64>().unwrap(), value);
                        self.insert_cache(left.parse::<u64>().unwrap(), value);

                        self.remove_cache(stone, value);
                    } else {
                        self.insert_cache(stone * 2024, value);
                        self.remove_cache(stone, value);
                    }
                });
        });

        self.stones.values().map(|v| *v as u64).sum()
    }

    fn insert_cache(&mut self, stone: u64, value: usize) {
        if let Some(seen) = self.stones.get_mut(&stone) {
            *seen += value;
        } else {
            self.stones.insert(stone, value);
        }
    }

    fn remove_cache(&mut self, stone: u64, value: usize) {
        if let Some(seen) = self.stones.get_mut(&stone) {
            *seen -= value;

            if *seen == 0 {
                let _ = self.stones.remove(&stone);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = Stones::from(input);

    Some(stones.blink(25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones = Stones::from(input);

    Some(stones.blink(75))
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_example;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(65_601_038_650_482));
    }
}
