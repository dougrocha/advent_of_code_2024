use std::ops::Mul;

advent_of_code_2024::solution!(13);

const BUTTON_A_TOKENS: u64 = 3;
const BUTTON_B_TOKENS: u64 = 1;

#[derive(Default, Debug, Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

#[derive(Default, Debug, Clone, Copy)]
struct ClawMachine {
    a: Vec2,
    b: Vec2,
    price: Vec2,
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    input
        .split("\n\n")
        .map(|x| {
            let mut buttons = x.lines().map(|x| {
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

            ClawMachine {
                a: buttons.next().expect("Expected button a"),
                b: buttons.next().expect("Expected button b"),
                price: buttons.next().expect("Expected price"),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let claw_machines = parse_input(input);

    let sum = claw_machines
        .iter()
        .map(|&x| {
            let button_a = x.a;
            let button_b = x.b;
            let price = x.price;

            let mut found_y = 0;

            for i in 0..100 {
                let res = button_a.y * ((price.x - button_b.x * i as f64) / button_a.x)
                    + (button_b.y * i as f64);

                if (res - price.y).abs() < f64::EPSILON {
                    found_y = i;
                    break;
                }
            }

            if found_y == 0 {
                return 0;
            }

            let mut found_x = 0;

            for i in 0..100 {
                let res = (button_a.x * i as f64) + (button_b.x * found_y as f64);

                if (res - price.x).abs() < f64::EPSILON {
                    found_x = i;
                    break;
                }
            }

            let button_a_cost = BUTTON_A_TOKENS * found_x;
            let button_b_cost = BUTTON_B_TOKENS * found_y;

            button_a_cost + button_b_cost
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
        assert_eq!(result, None);
    }
}
