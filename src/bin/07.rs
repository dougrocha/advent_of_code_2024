use itertools::{repeat_n, Itertools};

advent_of_code_2024::solution!(7);

#[derive(Debug)]
struct Line {
    lhs: u64,
    rhs: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Multiply,
    Add,
}

impl Operator {
    fn get_permutations(len: usize) -> Vec<Vec<Operator>> {
        repeat_n(vec![Operator::Multiply, Operator::Add], len - 1)
            .multi_cartesian_product()
            .collect()
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            dbg!(line);
            let (lhs, rhs) = line.split_once(":").unwrap();

            dbg!(lhs, rhs);

            let lhs = lhs.parse::<u64>().expect("lhs should be valid");
            let rhs: Vec<u64> = rhs
                .split_whitespace()
                .map(|x| x.parse::<_>().unwrap())
                .collect();

            Line { lhs, rhs }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let mut acc = 0;

    for Line { lhs, rhs } in input {
        let permutations = Operator::get_permutations(rhs.len());

        for permutation in permutations {
            let mut rhs_iter = rhs.iter();

            let mut sum = *rhs_iter.next().expect("should always have first number");

            for (rhs, operator) in rhs_iter.zip(permutation) {
                match operator {
                    Operator::Multiply => sum *= rhs,
                    Operator::Add => sum += rhs,
                }
            }

            if sum == lhs {
                acc += sum;
                break;
            }
        }
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_example;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, None);
    }
}
