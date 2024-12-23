advent_of_code_2024::solution!(13);

#[derive(Default, Debug, Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Default, Debug, Clone, Copy)]
struct ClawMachine {
    a: Vec2,
    b: Vec2,
    price: Vec2,
}
impl From<&str> for ClawMachine {
    fn from(value: &str) -> Self {
        let mut buttons = value.lines().map(|x| {
            let mut nums = x.split(',').filter_map(|x| {
                x.trim_start_matches(|x: char| !char::is_numeric(x))
                    .trim_end_matches(',')
                    .parse()
                    .ok()
            });

            Vec2 {
                x: nums.next().expect("Expected x coordinate"),
                y: nums.next().expect("Expected y coordinate"),
            }
        });

        Self {
            a: buttons.next().expect("Expected button a"),
            b: buttons.next().expect("Expected button b"),
            price: buttons.next().expect("Expected price"),
        }
    }
}

impl ClawMachine {
    const BUTTON_A_TOKENS: i64 = 3;
    const BUTTON_B_TOKENS: i64 = 1;

    /// Uses Cramers method
    /// https://www.youtube.com/watch?v=KOUjAzDyeZY
    ///
    /// ax(x) + bx(y) = price.x
    /// ay(x) + by(y) = price.y
    ///
    fn solve(&self) -> i64 {
        let a = self.a;
        let b = self.b;
        let price = self.price;

        let d = (a.x * b.y) - (a.y * b.x);

        if d == 0 {
            return 0; // No unique solution
        }

        let d_x = (price.x * b.y) - (price.y * b.x);
        let d_y = (a.x * price.y) - (a.y * price.x);

        if d_x % d != 0 || d_y % d != 0 {
            return 0;
        }

        let x = d_x / d;
        let y = d_y / d;

        let button_a_cost = Self::BUTTON_A_TOKENS * x;
        let button_b_cost = Self::BUTTON_B_TOKENS * y;

        button_a_cost + button_b_cost
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let sum = input
        .split("\n\n")
        .map(ClawMachine::from)
        .map(|claw_machine| claw_machine.solve())
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let sum = input
        .split("\n\n")
        .map(ClawMachine::from)
        .map(|mut claw_machine| {
            claw_machine.price.x += 10_000_000_000_000;
            claw_machine.price.y += 10_000_000_000_000;
            claw_machine.solve()
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_example;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(875318608908));
    }
}
