advent_of_code_2024::solution!(4);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Direction {
    fn to_offset(self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::UpRight => (-1, 1),
            Direction::UpLeft => (-1, -1),
            Direction::DownRight => (1, 1),
            Direction::DownLeft => (1, -1),
        }
    }
}

#[derive(Debug)]
struct XmasMap {
    arr: Vec<String>,
    width: usize,
    height: usize,
}

impl XmasMap {
    pub fn new(input: &str) -> Self {
        let input: Vec<&str> = input.lines().collect();
        let width = input.first().unwrap().len();
        let height = input.len();

        let arr: Vec<String> = input
            .iter()
            .flat_map(|x| x.chars())
            .map(String::from)
            .collect();

        Self { width, height, arr }
    }

    fn index_to_coords(&self, i: usize) -> (usize, usize) {
        let row = i / self.width;
        let col = i % self.height;

        (row, col)
    }

    fn coords_to_index(&self, row: i32, col: i32) -> Option<usize> {
        // assert here
        if row < 0 || row >= self.height as i32 || col < 0 || col >= self.width as i32 {
            return None;
        }

        let row = row as usize;
        let col = col as usize;

        Some(row * self.width + col)
    }

    /// Find XMAS match after m has been found
    fn search_direction(&self, index: usize, dir: Direction) -> bool {
        let (row_offset, col_offset) = dir.to_offset();

        let (row, col) = self.index_to_coords(index);
        let mut row = row as i32;
        let mut col = col as i32;

        for char in ["M", "A", "S"] {
            row += row_offset;
            col += col_offset;

            if let Some(next_index) = self.coords_to_index(row, col) {
                if self.arr[next_index] != char {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Finding X-MAS
    fn find_xmas(&self, index: usize) -> bool {
        let directions = [
            Direction::UpLeft,
            Direction::DownLeft,
            Direction::UpRight,
            Direction::DownRight,
        ];

        let surrounding = directions
            .iter()
            .flat_map(|dir| self.get_dir(index, *dir))
            .collect::<Vec<&str>>();
        let slice = surrounding.as_slice();

        matches!(
            slice,
            ["M", "M", "S", "S"]
                | ["S", "S", "M", "M"]
                | ["M", "S", "M", "S"]
                | ["S", "M", "S", "M"]
        )
    }

    fn get_dir(&self, index: usize, dir: Direction) -> Option<&str> {
        let (cur_row, cur_col) = self.index_to_coords(index);

        let (row_offset, col_offset) = dir.to_offset();

        if let Some(new_index) =
            self.coords_to_index(row_offset + cur_row as i32, col_offset + cur_col as i32)
        {
            return Some(&self.arr[new_index]);
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let map = XmasMap::new(input);

    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::UpRight,
        Direction::UpLeft,
        Direction::DownRight,
        Direction::DownLeft,
    ];

    let count = map
        .arr
        .iter()
        .enumerate()
        .flat_map(|(i, char)| {
            if char == "X" {
                directions
                    .iter()
                    .map(|dir| map.search_direction(i, *dir))
                    .filter(|x| *x)
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        })
        .count();

    Some(count as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    let map = XmasMap::new(input);

    let count = map
        .arr
        .iter()
        .enumerate()
        .map(|(i, char)| {
            if char == "A" {
                return map.find_xmas(i);
            }

            false
        })
        .filter(|x| *x)
        .count();

    Some(count as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_example;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(9));
    }
}
