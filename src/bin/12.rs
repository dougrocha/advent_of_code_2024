use advent_of_code_2024::{Coords, DIRECTIONS};
use itertools::Itertools;
use std::collections::VecDeque;

advent_of_code_2024::solution!(12);

#[derive(Debug)]
struct Grid {
    map: Vec<char>,
    width: usize,
    height: usize,
}

impl Coords for Grid {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self {
            map: value.chars().filter(|x| *x != '\n').collect(),
            width: value.lines().next().unwrap().len(),
            height: value.lines().count(),
        }
    }
}

#[derive(Debug)]
struct Region {
    area: u32,
    parameter: u32,
}

impl Region {
    fn get_price(&self) -> u32 {
        self.area * self.parameter
    }
}

impl Grid {
    fn calc_regions(&self) -> Vec<Region> {
        let mut res: Vec<Region> = Vec::new();

        let mut visited = vec![false; self.width * self.height];

        for index in 0..self.map.len() {
            if visited[index] {
                continue;
            }

            let mut queue: VecDeque<usize> = VecDeque::new();
            queue.push_back(index);

            let mut area = 0;
            let mut parameter = 0;

            while let Some(plant_index) = queue.pop_front() {
                if visited[plant_index] {
                    continue;
                }
                visited[plant_index] = true;

                area += 1;

                let plant = &self.map[plant_index];

                for direction in DIRECTIONS {
                    if let Some(pos) = self.move_index(plant_index, direction) {
                        let next_plant = &self.map[pos];

                        if plant == next_plant {
                            queue.push_back(pos);
                        } else {
                            parameter += 1;
                        }
                    } else {
                        parameter += 1;
                    }
                }
            }

            res.push(Region { area, parameter });
        }

        res
    }

    fn calc_regions_with_corners(&self) -> Vec<Region> {
        let mut res: Vec<Region> = Vec::new();

        let mut visited = vec![false; self.width * self.height];

        for index in 0..self.map.len() {
            if visited[index] {
                continue;
            }

            let mut queue: VecDeque<usize> = VecDeque::new();
            queue.push_back(index);

            let mut area = 0;
            let mut parameter = 0;

            while let Some(plant_index) = queue.pop_front() {
                if visited[plant_index] {
                    continue;
                }
                visited[plant_index] = true;

                area += 1;

                let plant = &self.map[plant_index];

                DIRECTIONS.iter().for_each(|&direction| {
                    if let Some(pos) = self.move_index(plant_index, direction) {
                        let next_plant = &self.map[pos];

                        if plant == next_plant {
                            queue.push_back(pos);
                        }
                    }
                });

                let corners = self.count_corners(plant_index);

                parameter += corners;
            }

            res.push(Region { area, parameter });
        }

        res
    }

    fn count_corners(&self, index: usize) -> u32 {
        let plant = self.map[index];

        let coords = self.index_to_coords(index);

        let mut count = 0;

        for ([x, y], [x1, y1]) in DIRECTIONS
            .iter()
            .map(|x| {
                let offset = x.to_offset();
                [offset.0, offset.1]
            })
            .circular_tuple_windows()
        {
            let test_a = self
                .coords_to_index(x + coords.0 as i32, y + coords.1 as i32)
                .map(|x| self.map[x])
                .is_some_and(|c| c == plant);
            let test_b = self
                .coords_to_index(x1 + coords.0 as i32, y1 + coords.1 as i32)
                .map(|x| self.map[x])
                .is_some_and(|c| c == plant);

            if test_a
                && test_b
                && self
                    .coords_to_index(x + x1 + coords.0 as i32, y + y1 + coords.1 as i32)
                    .map(|x| self.map[x])
                    .is_some_and(|c| c != plant)
            {
                count += 1;
                continue;
            }

            if !test_a && !test_b {
                count += 1;
            }
        }

        count
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from(input);

    Some(grid.calc_regions().iter().map(|x| x.get_price()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from(input);

    Some(
        grid.calc_regions_with_corners()
            .iter()
            .map(|x| x.get_price())
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(1206));
    }
}
