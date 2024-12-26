use advent_of_code_2024::IVec2;
use itertools::Itertools;

advent_of_code_2024::solution!(14);

#[derive(Debug)]
struct Robot {
    pos: IVec2,
    velocity: IVec2,
}

struct Headquarters {
    robots: Vec<Robot>,
}

impl Headquarters {
    const TIME_DURATION: i32 = 100;
    const WIDTH: usize = if cfg!(test) { 11 } else { 101 };
    const HEIGHT: usize = if cfg!(test) { 7 } else { 103 };

    fn move_robots_by_time(&mut self) {
        for robot in &mut self.robots.iter_mut() {
            robot.pos.x = (robot.pos.x + robot.velocity.x * Self::TIME_DURATION)
                .rem_euclid(Self::WIDTH as i32);
            robot.pos.y = (robot.pos.y + robot.velocity.y * Self::TIME_DURATION)
                .rem_euclid(Self::HEIGHT as i32);
        }
    }

    fn safety_factor(&self) -> usize {
        let quadrants = [
            ((0..Self::WIDTH / 2), (0..Self::HEIGHT / 2)),
            ((Self::WIDTH / 2..Self::WIDTH), (0..Self::HEIGHT / 2)),
            ((0..Self::WIDTH / 2), (Self::HEIGHT / 2..Self::HEIGHT)),
            (
                (Self::WIDTH / 2..Self::WIDTH),
                (Self::HEIGHT / 2..Self::HEIGHT),
            ),
        ];

        quadrants
            .iter()
            .map(|(x_range, y_range)| {
                self.robots
                    .iter()
                    .filter(|robot| {
                        let (x, y) = (robot.pos.x as usize, robot.pos.y as usize);

                        if x == Self::WIDTH / 2 || y == Self::HEIGHT / 2 {
                            return false;
                        }

                        if x_range.contains(&x) && y_range.contains(&y) {
                            return true;
                        }

                        false
                    })
                    .count()
            })
            .product()
    }

    fn find_tree(&mut self) -> u32 {
        let mut seconds = 0;
        loop {
            for robot in &mut self.robots.iter_mut() {
                robot.pos.x = (robot.pos.x + robot.velocity.x).rem_euclid(Self::WIDTH as i32);
                robot.pos.y = (robot.pos.y + robot.velocity.y).rem_euclid(Self::HEIGHT as i32);
            }

            seconds += 1;
            if self.test_tree(&self.robots) {
                break;
            }
        }

        seconds
    }

    // all positions should somehow be unique?
    fn test_tree(&self, robots: &[Robot]) -> bool {
        robots.iter().map(|Robot { pos, .. }| pos).all_unique()
    }
}

impl From<&str> for Headquarters {
    fn from(value: &str) -> Self {
        Self {
            robots: value
                .lines()
                .map(|line| {
                    let (left, right) = line.split_once(' ').unwrap();

                    let parse_coords = |part: &str| -> IVec2 {
                        let mut nums = part[2..].split(',').map(|x| x.parse::<i32>().unwrap());

                        IVec2 {
                            x: nums.next().unwrap(),
                            y: nums.next().unwrap(),
                        }
                    };

                    Robot {
                        pos: parse_coords(left),
                        velocity: parse_coords(right),
                    }
                })
                .collect(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hq = Headquarters::from(input);
    hq.move_robots_by_time();
    Some(hq.safety_factor() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hq = Headquarters::from(input);
    Some(hq.find_tree())
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_example;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(1));
    }
}
