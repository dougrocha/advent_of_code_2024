advent_of_code_2024::solution!(5);

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (rules, pages) = input.split_once("\n\n").expect("both parts to exist");

    let rules: Vec<(i32, i32)> = rules
        .lines()
        .map(|line| {
            let nums = line.split_once("|").expect("where did the pipe go?");
            (nums.0.parse().unwrap(), nums.1.parse().unwrap())
        })
        .collect();

    let pages_list: Vec<Vec<i32>> = pages
        .lines()
        .map(|line| {
            line.split_terminator(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    (rules, pages_list)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages_list) = parse_input(input);

    let valid = pages_list
        .iter()
        .flat_map(|pages| {
            for (before, after) in &rules {
                let filtered: Vec<i32> = pages
                    .iter()
                    .filter_map(|x| {
                        if x == before || x == after {
                            Some(*x)
                        } else {
                            None
                        }
                    })
                    .collect();

                if filtered.len() == 2 && (*before != filtered[0] || *after != filtered[1]) {
                    return None;
                }
            }

            Some(pages)
        })
        .collect::<Vec<_>>();

    Some(valid.iter().map(|x| x[x.len() / 2]).sum::<i32>() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, pages_list) = parse_input(input);

    let not_valid = pages_list
        .iter()
        .flat_map(|pages| {
            for (before, after) in &rules {
                let filtered: Vec<i32> = pages
                    .iter()
                    .filter_map(|x| {
                        if x == before || x == after {
                            Some(*x)
                        } else {
                            None
                        }
                    })
                    .collect();

                if filtered.len() == 2 && (*before != filtered[0] || *after != filtered[1]) {
                    return Some(pages);
                }
            }

            None
        })
        .collect::<Vec<_>>();

    // check the not valids and switch places using the rules
    // for each rule, if it is not valid then swap
    // swap until valid and return middle index sum like part

    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_example;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, None);
    }
}
