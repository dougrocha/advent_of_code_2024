use advent_of_code_2024::{Coords, DIAGONAL_DIRECTIONS, DIRECTIONS};
use itertools::chain;
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
    plant: char,
    area: u32,
    parameter: u32,
    corners: Vec<(usize, usize)>,
}

impl Region {
    fn get_price(&self) -> u32 {
        self.area * self.parameter
    }

    fn calc_parameter_with_corners(&self) -> Self {
        let parameter = 0;

        Self {
            plant: self.plant,
            area: self.area,
            parameter,
            corners: vec![],
        }
    }
}

impl Grid {
    fn calc_area_parameter(&self) -> Vec<Region> {
        let mut res: Vec<Region> = Vec::new();

        let mut visited = vec![false; self.width * self.height];

        for (index, plant) in self.map.iter().enumerate() {
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

            res.push(Region {
                plant: *plant,
                area,
                parameter,
                corners: vec![],
            });
        }

        res
    }

    fn calc_sides(&self) -> Vec<Region> {
        let mut res: Vec<Region> = Vec::new();

        let mut visited = vec![false; self.width * self.height];

        for (index, plant) in self.map.iter().enumerate() {
            if visited[index] {
                continue;
            }

            let mut queue: VecDeque<usize> = VecDeque::new();
            queue.push_back(index);

            let mut area = 0;
            let mut corners = Vec::new();

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

                if self.is_corner(plant_index) {
                    corners.push(self.index_to_coords(plant_index));
                }
            }

            res.push(Region {
                plant: *plant,
                area,
                parameter: 0,
                corners,
            });
        }

        res
    }

    fn is_corner(&self, index: usize) -> bool {
        let plant = self.map[index];

        let arr: Vec<bool> = chain(DIRECTIONS, DIAGONAL_DIRECTIONS)
            .map(|direction| {
                if let Some(pos) = self.move_index(index, direction) {
                    let next_plant = self.map[pos];

                    if plant == next_plant {
                        return false;
                    }
                }

                true
            })
            .collect();

        let [up, down, left, right, up_left, down_left, up_right, down_right]: [bool; 8] =
            arr.try_into().ok().unwrap();

        if (down || up) && (left || right) {
            dbg!('1');
            return true;
        }
        if up_left && !up && !left && !down && !right {
            dbg!('2');
            return true;
        }
        if up_right && !up && !right && !down && !left {
            dbg!('3');
            return true;
        }
        if down_left && !down && !left && !up && !right {
            dbg!('4');
            return true;
        }
        if down_right && !down && !right && !up && !left {
            dbg!('5');
            return true;
        }

        false
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from(input);

    let res = grid.calc_area_parameter();

    Some(res.iter().map(|x| x.get_price()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from(input);

    let res = grid.calc_sides();

    Some(
        res.iter()
            .map(|x| x.calc_parameter_with_corners().get_price())
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
