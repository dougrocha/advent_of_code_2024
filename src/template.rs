use crate::{day::Day, download_day};
use std::fs;

pub fn create_files(day: Day) {
    let input_path = format!("./inputs/{day}.txt");
    let example_path = format!("./examples/{day}.txt");
    let module_path = format!("./src/bin/{day}.rs");

    let module_template = MODULE_TEMPLATE_TEXT.replace("%%DAY%%", format!("{day}").as_str());
    let _ = fs::write(module_path, module_template);

    let downloaded_input = download_day(day).expect("day to be downloaded");
    let _ = fs::write(input_path, downloaded_input);

    let _ = fs::write(example_path, "");
    println!("Remember to fill out example path");
}

const MODULE_TEMPLATE_TEXT: &str = r"
advent_of_code_2024::solution!(%%DAY%%);

pub fn part_one(input: &str) -> Option<u32> {
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, None);
    }
}
";
