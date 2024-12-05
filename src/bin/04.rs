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

    count: u32,
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

        Self {
            width,
            height,
            arr,
            count: 0,
        }
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

    /// find xmas match after m has been found
    fn find_m(&mut self, index: usize) {
        let directions = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ];

        for dir in directions {
            let (row_offset, col_offset) = dir.to_offset();

            let (row, col) = self.index_to_coords(index);

            if let Some(index) =
                self.coords_to_index(row as i32 + row_offset, col as i32 + col_offset)
            {
                let next_letter = &self.arr[index];

                if next_letter.as_str() == "M" {
                    let (row, col) = self.index_to_coords(index);
                    if let Some(next_index) =
                        self.coords_to_index(row as i32 + row_offset, col as i32 + col_offset)
                    {
                        self.find_as(next_index, &dir);
                    }
                }
            }
        }
    }

    fn find_as(&mut self, index: usize, dir: &Direction) {
        let (row_offset, col_offset) = dir.to_offset();

        let (row, col) = self.index_to_coords(index);

        let next_letter = &self.arr[index];

        if next_letter.as_str() == "A" {
            if let Some(next_index) =
                self.coords_to_index(row as i32 + row_offset, col as i32 + col_offset)
            {
                let this = &mut *self;
                let next_letter = &this.arr[next_index];

                if next_letter.as_str() == "S" {
                    this.count += 1;
                };
            }
        }
    }

    fn find_xmas(&mut self, index: usize) {
        let directions = [
            Direction::UpLeft,
            Direction::DownLeft,
            Direction::UpRight,
            Direction::DownRight,
        ];

        let vals = directions
            .iter()
            .flat_map(|dir| self.get_dir(index, *dir))
            .collect::<Vec<String>>();

        if vals.len() == 4 {
            let vals = vals.iter().map(|x| x.as_str()).collect::<Vec<_>>();
            match vals[..] {
                ["M", "M", "S", "S"] => {
                    let (row, col) = self.index_to_coords(index);
                    println!("{} - ({:?}): {:?}", index + 1, (row + 1, col + 1), vals);
                    self.count += 1;
                }
                ["S", "S", "M", "M"] => {
                    let (row, col) = self.index_to_coords(index);
                    println!("{} - ({:?}): {:?}", index + 1, (row + 1, col + 1), vals);
                    self.count += 1;
                }
                ["M", "S", "M", "S"] => {
                    let (row, col) = self.index_to_coords(index);
                    println!("{} - ({:?}): {:?}", index + 1, (row + 1, col + 1), vals);
                    self.count += 1;
                }
                ["S", "M", "S", "M"] => {
                    let (row, col) = self.index_to_coords(index);
                    println!("{} - ({:?}): {:?}", index + 1, (row + 1, col + 1), vals);
                    self.count += 1;
                }
                _ => {}
            }
        }
    }

    fn get_dir(&self, index: usize, dir: Direction) -> Option<String> {
        let (cur_row, cur_col) = self.index_to_coords(index);

        let (row_offset, col_offset) = dir.to_offset();

        if let Some(new_index) =
            self.coords_to_index(row_offset + cur_row as i32, col_offset + cur_col as i32)
        {
            return Some(self.arr[new_index].clone());
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = XmasMap::new(input);

    let mut x_indexs = Vec::new();

    for (i, char) in map.arr.iter().enumerate() {
        if char == "X" {
            x_indexs.push(i);
        }
    }

    for i in x_indexs {
        map.find_m(i);
    }

    Some(map.count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = XmasMap::new(input);

    let mut x_indexs = Vec::new();

    for (i, char) in map.arr.iter().enumerate() {
        if char == "A" {
            x_indexs.push(i);
        }
    }

    for i in x_indexs {
        map.find_xmas(i);
    }

    Some(map.count)
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
