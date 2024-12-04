advent_of_code_2024::solution!(3);

struct MulParser<'a> {
    rest: &'a str,
}

impl Iterator for MulParser<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.rest.find("mul(") {
            self.rest = &self.rest[index + 3..];

            let mut read = 0;

            if self.rest.starts_with('(') {
                let chars = self.rest.chars();

                for c in chars {
                    match c {
                        '0'..='9' | ',' | '(' => {
                            read += 1;
                        }
                        ')' => {
                            read += 1;
                            break;
                        }
                        _ => {
                            self.rest = &self.rest[read..];
                            return Some(0);
                        }
                    }
                }
            }

            let s = &self.rest[1..read - 1]
                .split(',')
                .map(|x| x.parse::<u32>().unwrap_or(0))
                .collect::<Vec<_>>();

            return Some(s[0] * s[1]);
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let parser = MulParser { rest: input };

    let sum = parser.sum::<u32>();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parser = MulParser { rest: input };

    let sum = parser.sum::<u32>();

    Some(sum)
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
        let result =
            part_two("&xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
