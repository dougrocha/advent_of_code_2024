advent_of_code_2024::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut rest = input;

    let mut sum = 0;

    'outer: loop {
        let mut chars = rest.chars();

        if chars.next().is_none() {
            break;
        }

        rest = chars.as_str();

        let mul = match &rest.get(..4) {
            Some(mul) => *mul,
            None => break,
        };

        if mul == "mul(" {
            rest = &rest[3..];

            let mut read = 0;

            if rest.starts_with('(') {
                chars = rest.chars();

                loop {
                    let c = chars.next()?;

                    match c {
                        '0'..='9' | ',' | '(' => {
                            read += 1;
                        }
                        ')' => {
                            read += 1;
                            break;
                        }
                        _ => {
                            rest = &rest[read..];
                            continue 'outer;
                        }
                    }
                }
            }

            let s = &rest[1..read - 1]
                .split(',')
                .map(|x| x.parse::<u32>().unwrap_or(0))
                .collect::<Vec<_>>();

            sum += s[0] * s[1];
        }
    }

    Some(sum)
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, None);
    }
}
