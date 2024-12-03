use itertools::Itertools;

advent_of_code_2024::solution!(2);

#[derive(Debug)]
enum Direction {
    Asc,
    Desc,
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let numbers = numbers.as_slice();

            is_safe(numbers)
        })
        .filter(|x| *x);

    Some(res.count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let numbers = numbers.as_slice();

            let mut safe = true;

            // TODO
            for i in 0..numbers.len() {
                let mut copy = Vec::from(numbers);
                let _ = copy.remove(i);

                safe = is_safe(&copy);

                if safe {
                    break;
                }
            }

            safe
        })
        .filter(|x| *x);

    Some(res.count() as u32)
}

fn is_safe(numbers: &[i32]) -> bool {
    let mut direction = None;

    numbers
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| b - a)
        .all(|diff| {
            if diff.abs() > 3 {
                return false;
            }

            if diff == 0 {
                return false;
            }

            match direction {
                Some(Direction::Asc) => {
                    if diff < 0 {
                        return false;
                    }
                    true
                }
                Some(Direction::Desc) => {
                    if diff > 0 {
                        return false;
                    }
                    true
                }
                None => {
                    if diff < 0 {
                        direction = Some(Direction::Desc);
                    }
                    if diff > 0 {
                        direction = Some(Direction::Asc);
                    }
                    true
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_example;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(4));
    }
}
