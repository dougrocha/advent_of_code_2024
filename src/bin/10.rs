use itertools::Itertools;
use std::collections::VecDeque;

advent_of_code_2024::solution!(10);

#[derive(Debug)]
struct Map {
    map: Vec<u32>,
    width: usize,
    height: usize,
}

impl Map {
    fn find_trailheads(&self) -> Vec<usize> {
        self.map
            .iter()
            .enumerate()
            .filter(|&(_, &x)| x == 0)
            .map(|(index, _)| index)
            .collect::<Vec<usize>>()
    }

    fn index_to_coords(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    fn coords_to_index(&self, row: i32, col: i32) -> Option<usize> {
        if row < 0 || row >= self.height as i32 || col < 0 || col >= self.width as i32 {
            return None;
        }

        let row = row as usize;
        let col = col as usize;

        Some(row * self.width + col)
    }

    fn compute_trailhead(&self, start: usize) -> Vec<usize> {
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(start);

        let mut scored = Vec::new();

        while let Some(cur_pos) = queue.pop_front() {
            let cur_value = self.map[cur_pos];

            let (cur_x, cur_y) = self.index_to_coords(cur_pos);
            let cur_x = cur_x as i32;
            let cur_y = cur_y as i32;

            if cur_value == 9 {
                scored.push(cur_pos);
            }

            // left
            if let Some(pos) = self.coords_to_index(cur_x, cur_y - 1) {
                let val = self.map[pos];
                if cur_value + 1 == val {
                    queue.push_back(pos);
                }
            }

            // right
            if let Some(pos) = self.coords_to_index(cur_x, cur_y + 1) {
                let val = self.map[pos];
                if cur_value + 1 == val {
                    queue.push_back(pos);
                }
            }

            // up
            if let Some(pos) = self.coords_to_index(cur_x - 1, cur_y) {
                let val = self.map[pos];
                if cur_value + 1 == val {
                    queue.push_back(pos);
                }
            }

            // down
            if let Some(pos) = self.coords_to_index(cur_x + 1, cur_y) {
                let val = self.map[pos];
                if cur_value + 1 == val {
                    queue.push_back(pos);
                }
            }
        }

        scored
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Self {
            map: value
                .chars()
                .filter(|x| *x != '\n')
                .map(|x| x.to_digit(10).unwrap())
                .collect(),
            width: value.lines().next().unwrap().len(),
            height: value.lines().count(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from(input);

    let starts = map.find_trailheads();

    Some(
        starts
            .iter()
            .map(|start| {
                map.compute_trailhead(*start)
                    .iter()
                    .unique()
                    .collect_vec()
                    .len() as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::from(input);

    let starts = map.find_trailheads();

    Some(
        starts
            .iter()
            .map(|start| map.compute_trailhead(*start).iter().collect_vec().len() as u32)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(81));
    }
}
