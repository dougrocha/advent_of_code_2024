pub mod day;
pub mod math;
pub mod template;

pub use math::*;
pub use template::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    DownLeft,
    UpRight,
    DownRight,
}

impl Direction {
    pub fn to_offset(self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::UpLeft => (-1, -1),
            Direction::DownLeft => (1, -1),
            Direction::UpRight => (-1, 1),
            Direction::DownRight => (1, 1),
        }
    }
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

pub const DIAGONAL_DIRECTIONS: [Direction; 4] = [
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownRight,
    Direction::DownLeft,
];

pub trait Coords {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn index_to_coords(&self, index: usize) -> (usize, usize) {
        (index / self.width(), index % self.width())
    }

    fn coords_to_index(&self, row: i32, col: i32) -> Option<usize> {
        if row < 0 || row >= self.height() as i32 || col < 0 || col >= self.width() as i32 {
            return None;
        }

        let row = row as usize;
        let col = col as usize;

        Some(row * self.width() + col)
    }

    fn move_index(&self, index: usize, direction: Direction) -> Option<usize> {
        let offset = direction.to_offset();
        let coords = self.index_to_coords(index);

        let mut row = coords.0 as i32;
        let mut col = coords.1 as i32;

        row += offset.0;
        col += offset.1;

        self.coords_to_index(row, col)
    }
}
