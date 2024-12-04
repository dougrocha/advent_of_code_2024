advent_of_code_2024::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut rest = input;

    let numbers: Vec<(u32, u32)> = Vec::new();

    loop {
        let mut chars = rest.chars();

        let next = match chars.next() {
            Some(char) => char,
            None => break,
        };
        rest = chars.as_str();

        let mul = &rest[..4];
        if mul == "mul(" {
            rest = &rest[3..];

            let mut read = 0;

            dbg!(rest);

            if rest.starts_with('(') {
                rest = &rest[1..];
                chars = rest.chars();

                loop {
                    let c = chars.next()?;
                    match c {
                        '0'..='9' => {
                            dbg!("number", c);
                        }
                        ',' => {
                            dbg!("comma", c);
                        }
                        ')' => {
                            dbg!("ending", c);
                            break;
                        }
                        _ => break,
                    }
                    read += 1;
                }
            }

            dbg!(&rest[..read]); // "8,5"

            // TODO: Split this string and compute math
        }
    }

    None
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
