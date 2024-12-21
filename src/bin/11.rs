use std::collections::HashMap;

advent_of_code_2024::solution!(11);

struct Stones {
    arr: Vec<u64>,
    cache: HashMap<(u64, usize), u64>,
}
impl From<&str> for Stones {
    fn from(value: &str) -> Self {
        Self {
            arr: value
                .trim()
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect(),
            cache: HashMap::default(),
        }
    }
}

impl Stones {
    fn blink(&self, num: u64, blinks: usize) -> Vec<u64> {
        let mut nums = vec![num];

        for _ in 0..blinks {
            let mut current = Vec::new();

            nums.iter().for_each(|num| {
                if *num == 0 {
                    current.push(1);
                } else if num.to_string().len() % 2 == 0 {
                    let str = num.to_string();
                    let (left, right) = str.split_at(str.len() / 2);

                    current.push(left.parse::<u64>().unwrap());
                    current.push(right.parse::<u64>().unwrap());
                } else {
                    current.push(*num * 2024);
                }
            });

            nums = current;
        }

        nums
    }

    fn blink_with_cache(&self) -> u64 {
        0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = Stones::from(input);

    let sum = stones.arr.iter().flat_map(|x| stones.blink(*x, 25)).count();

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = Stones::from(input);

    Some(0)
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
        assert_eq!(result, None);
    }
}
