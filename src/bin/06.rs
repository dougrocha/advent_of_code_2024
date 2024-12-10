advent_of_code_2024::solution!(6);

#[derive(Debug, Clone, Copy)]
enum Direction {
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
                        cur_direction = match cur_direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        };
                    }

                    let offset = &cur_direction.to_offset();
                    x += offset.0;
                    y += offset.1;
                } else {
                    break;
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut map = Map::new(input);

    let start = map.get_start();

    map.parse_path(start);

    Some(map.arr.iter().filter(|x| **x == 'X').count() as i32)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, None);
    }
}
