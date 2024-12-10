advent_of_code_2024::solution!(6);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_offset(self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

struct Map {
    arr: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let arr: Vec<char> = input.lines().flat_map(|x| x.chars()).collect();

        Self { arr, width, height }
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

    fn get_start(&self) -> (usize, usize) {
        let index = self.arr.iter().position(|x| *x == '^').unwrap();
        self.index_to_coords(index)
    }

    fn parse_path(&mut self, start: (usize, usize)) {
        let mut cur_direction = Direction::Up;

        let mut x = start.0 as i32;
        let mut y = start.1 as i32;

        loop {
            if let Some(cur_spot) = self.coords_to_index(x, y) {
                self.arr[cur_spot] = 'X';

                let offset = &cur_direction.to_offset();
                let check_x = x + offset.0;
                let check_y = y + offset.1;

                if let Some(to_check) = self.coords_to_index(check_x, check_y) {
                    if self.arr[to_check] == '#' {
                        cur_direction = cur_direction.turn_right();
                    } else {
                        x = check_x;
                        y = check_y;
                    }
                } else {
                    break;
                }
            }
        }
    }

    fn parse_cycle_paths(&mut self, start: (usize, usize)) -> i32 {
        let mut dir = Direction::Up;

        let mut x = start.0 as i32;
        let mut y = start.1 as i32;

        let mut visited = vec![vec![false; self.width]; self.height];
        visited[x as usize][y as usize] = true;

        loop {
            if self.coords_to_index(x, y).is_some() {
                let offset = dir.to_offset();
                let new_x = x + offset.0;
                let new_y = y + offset.1;

                if let Some(next_index) = self.coords_to_index(new_x, new_y) {
                    if self.arr[next_index] == '#' {
                        dir = dir.turn_right();
                    } else {
                        x = new_x;
                        y = new_y;

                        visited[x as usize][y as usize] = true;
                    }
                } else {
                    break;
                }
            }
        }

        visited[start.0][start.1] = false;

        let res = visited
            .iter()
            .flatten()
            .enumerate()
            .filter_map(|(index, val)| {
                let coords = self.index_to_coords(index);

                if *val {
                    Some(coords)
                } else {
                    None
                }
            })
            .filter(|wall_coords| {
                let wall_index = self
                    .coords_to_index(wall_coords.0 as i32, wall_coords.1 as i32)
                    .unwrap();

                let mut x = start.0 as i32;
                let mut y = start.1 as i32;
                let mut dir = Direction::Up;

                let mut visited =
                    vec![vec![(false, Direction::default()); self.width]; self.height];
                visited[x as usize][y as usize] = (true, dir);

                loop {
                    if self.coords_to_index(x, y).is_some() {
                        let offset = dir.to_offset();
                        let new_x = x + offset.0;
                        let new_y = y + offset.1;

                        if let Some(next_index) = self.coords_to_index(new_x, new_y) {
                            let next_spot = visited[new_x as usize][new_y as usize];
                            if next_spot.0 && next_spot.1 == dir {
                                break true;
                            }

                            if self.arr[next_index] == '#' || wall_index == next_index {
                                dir = dir.turn_right();
                            } else {
                                x = new_x;
                                y = new_y;

                                visited[x as usize][y as usize] = (true, dir);
                            }
                        } else {
                            break false;
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        res.len() as i32
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut map = Map::new(input);

    let start = map.get_start();

    map.parse_path(start);

    Some(map.arr.iter().filter(|x| **x == 'X').count() as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut map = Map::new(input);

    let start = map.get_start();

    Some(map.parse_cycle_paths(start))
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_example;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(6));
    }
}
