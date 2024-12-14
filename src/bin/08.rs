use itertools::Itertools;
use std::collections::HashMap;

advent_of_code_2024::solution!(8);

fn parse_antennas(input: &str) -> HashMap<char, Vec<(usize, usize)>> {
    let width = input.lines().next().unwrap().len();

    let mut hash_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    input
        .chars()
        .filter(|x| *x != '\n')
        .enumerate()
        .filter(|(_, x)| *x != '.')
        .for_each(|(index, x)| {
            let coords = (index / width, index % width);

            if let Some(val) = hash_map.get_mut(&x) {
                val.push(coords);
            } else {
                hash_map.insert(x, Vec::from([coords]));
            }
        });

    hash_map
}

fn coords_to_index(width: usize, height: usize, row: i32, col: i32) -> Option<usize> {
    if row < 0 || row >= height as i32 || col < 0 || col >= width as i32 {
        return None;
    }
    let row = row as usize;
    let col = col as usize;

    Some(row * width + col)
}

pub fn part_one(input: &str) -> Option<i32> {
    let map = parse_antennas(input);

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let res = input.chars().filter(|x| *x != '\n').collect_vec();
    let mut result: String = res.into_iter().collect();

    for (_, arr) in map.iter() {
        for antenna_pair in arr.iter().combinations(2) {
            let first = antenna_pair[0];
            let second = antenna_pair[1];

            let diff_x = first.0 as i32 - second.0 as i32;
            let diff_y = first.1 as i32 - second.1 as i32;

            {
                let new_x = first.0 as i32 + diff_x;
                let new_y = first.1 as i32 + diff_y;

                let (row, col) = (new_x, new_y);

                if let Some(index) = coords_to_index(width, height, row, col) {
                    if result.get(index..index + 1).is_some() {
                        result.replace_range(index..index + 1, "#");
                    }
                }
            }

            {
                let new_x = second.0 as i32 - diff_x;
                let new_y = second.1 as i32 - diff_y;

                let (row, col) = (new_x, new_y);

                if let Some(index) = coords_to_index(width, height, row, col) {
                    if result.get(index..index + 1).is_some() {
                        result.replace_range(index..index + 1, "#");
                    }
                }
            }
        }
    }

    for line in &result.chars().chunks(width) {
        println!("{}", line.collect::<String>());
    }

    let sum = result.chars().filter(|x| *x == '#').count();

    Some(sum as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_example;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, None);
    }
}
