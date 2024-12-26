use advent_of_code_2024::IVec2;

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

    fn solve(&mut self) {
        for robot in &mut self.robots.iter_mut() {
            let mut new_x = robot.pos.x + robot.velocity.x * Self::TIME_DURATION;
            let mut new_y = robot.pos.y + robot.velocity.y * Self::TIME_DURATION;

            if new_y < 0 {
                new_y = (new_y % Self::HEIGHT as i32) + Self::HEIGHT as i32;
            }

            if new_x < 0 {
                new_x = (new_x % Self::WIDTH as i32) + Self::WIDTH as i32;
            }

            let new_x = new_x % Self::WIDTH as i32;
            let new_y = new_y % Self::HEIGHT as i32;

            robot.pos.x = new_x;
            robot.pos.y = new_y;
        }
    }

    fn safety_factor(&self) -> u32 {
        let top_left = ((0, 0), (Self::WIDTH / 2, Self::HEIGHT / 2));
        let top_right = ((Self::WIDTH / 2, 0), (Self::WIDTH, Self::HEIGHT / 2));
        let bottom_left = ((0, Self::HEIGHT / 2), (Self::WIDTH / 2, Self::HEIGHT));
        let bottom_right = (
            (Self::WIDTH / 2, Self::HEIGHT / 2),
            (Self::WIDTH, Self::HEIGHT),
        );

        let mut total = 1;

        for (start, end) in &[top_left, top_right, bottom_left, bottom_right] {
            let mut cur_count = 0;
            for robot in &self.robots {
                let x = robot.pos.x as usize;
                let y = robot.pos.y as usize;

                if x == Self::WIDTH / 2 || y == Self::HEIGHT / 2 {
                    continue;
                }

                if start.0 <= x && start.1 <= y && x < end.0 && y < end.1 {
                    cur_count += 1;
                }
            }

            total *= cur_count;
        }

        total
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
    hq.solve();
    Some(hq.safety_factor())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
