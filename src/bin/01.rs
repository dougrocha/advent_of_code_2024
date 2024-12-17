advent_of_code_2024::solution!(1);

use std::{iter::zip, ops::Mul};

fn parse_data(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first_col = Vec::new();
    let mut second_col = Vec::new();

    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|char| char.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .for_each(|line| {
            first_col.push(line[0]);
            second_col.push(line[1]);
        });

    (first_col, second_col)
}

fn part_one(input: &str) -> Option<i32> {
    let (mut first_col, mut second_col) = parse_data(input);

    first_col.sort();
    second_col.sort();

    Some(
        zip(first_col, second_col)
            .map(|(f, s)| match f.cmp(&s) {
                std::cmp::Ordering::Less => s - f,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => f - s,
            })
            .sum(),
    )
}

fn part_two(input: &str) -> Option<i32> {
    let (first_col, second_col) = parse_data(input);

    Some(
        first_col
            .iter()
            .map(|f| (second_col.iter().filter(|s| *s == f).count() as i32).mul(*f))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_example;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(31));
    }
}
