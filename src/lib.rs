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
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::UpLeft => (-1, -1),
            Direction::DownLeft => (-1, 1),
            Direction::UpRight => (1, -1),
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
        (index % self.width(), index / self.width())
    }

    fn coords_to_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.width() as i32 || y < 0 || y >= self.height() as i32 {
            return None;
        }

        Some(x as usize + y as usize * self.height())
    }

    fn move_index(&self, index: usize, direction: Direction) -> Option<usize> {
        let (x, y) = self.index_to_coords(index);
        let (offset_x, offset_y) = direction.to_offset();

        self.coords_to_index(x as i32 + offset_x, y as i32 + offset_y)
    }
}
