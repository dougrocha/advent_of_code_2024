use itertools::{repeat_n, Itertools};

advent_of_code_2024::solution!(7);

#[derive(Debug)]
struct Line {
    lhs: u64,
    rhs: Vec<u64>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operator {
    Multiply,
    Add,
    Concat,
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(":").unwrap();

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

    let res = input.iter().fold(0, |acc, Line { lhs, rhs }| {
        let permutations: Vec<Vec<Operator>> =
            repeat_n(vec![Operator::Multiply, Operator::Add], rhs.len() - 1)
                .multi_cartesian_product()
                .collect();

        for permutation in permutations {
            let mut rhs_iter = rhs.iter();

            let mut sum = *rhs_iter.next().expect("should always have first number");

            for (rhs, operator) in rhs_iter.zip(permutation) {
                match operator {
                    Operator::Multiply => sum *= rhs,
                    Operator::Add => sum += rhs,
                    _ => {}
                }
            }

            if sum == *lhs {
                return acc + sum;
            }
        }

        acc
    });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let res = input.iter().fold(0, |acc, Line { lhs, rhs }| {
        let permutations: Vec<Vec<Operator>> = repeat_n(
            vec![Operator::Multiply, Operator::Add, Operator::Concat],
            rhs.len() - 1,
        )
        .multi_cartesian_product()
        .collect();

        for permutation in permutations {
            let mut rhs_iter = rhs.iter();

            let mut sum = *rhs_iter.next().expect("should always have first number");

            for (num, operator) in rhs_iter.zip(&permutation) {
                match operator {
                    Operator::Multiply => sum *= num,
                    Operator::Add => sum += num,
                    Operator::Concat => {
                        let mut sum_str = sum.to_string();
                        let num_str = num.to_string();
                        sum_str.push_str(&num_str);

                        sum = sum_str
                            .parse::<u64>()
                            .expect("concatenated string should be a valid number")
                    }
                }
            }

            if sum == *lhs {
                return acc + sum;
            }
        }

        acc
    });

    Some(res)
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
        assert_eq!(result, Some(11387));
    }
}
