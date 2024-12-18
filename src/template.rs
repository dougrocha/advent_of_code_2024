use crate::day::Day;
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    time::Duration,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("network error: {0}")]
    NetworkError(#[from] Box<ureq::Error>),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

fn get_path(path: &str, day: Day) -> PathBuf {
    let path = format!("./{path}/{day}.txt");
    let path = Path::new(&path);

    path.to_path_buf()
}

fn get_session_token() -> Result<String> {
    Ok(fs::read_to_string("./session_token.txt")?)
}

fn download_day(day: Day) -> Result<String> {
    let session_token = get_session_token()?;
    let cookie = format!("session={}", &session_token.trim_end());

    let href = format!(
        "https://adventofcode.com/2024/day/{}/input",
        day.into_inner()
    );
    let body = ureq::get(&href)
        .set("cookie", &cookie)
        .timeout(Duration::from_secs(3))
        .call()
        .map_err(Box::new)?;

    let mut buf = Vec::new();
    let _ = body.into_reader().read_to_end(&mut buf);

    Ok(String::from_utf8(buf).expect("buf turn into string"))
}

pub fn read_input(day: Day) -> String {
    let path = get_path("inputs", day);

    if let Ok(input) = fs::read_to_string(&path) {
        return input;
    }

    let input = download_day(day).expect("day to be downloaded");
    let _ = fs::write(path, &input);
    input
}

pub fn read_example(day: Day) -> String {
    let path = get_path("examples", day);

    let input = fs::read_to_string(&path);
    input.expect("example file to be opened")
}

pub fn execute_parts<F1, F2, T>(input: &str, day: Day, part_one: F1, part_two: F2)
where
    F1: FnOnce(&str) -> Option<T>,
    F2: FnOnce(&str) -> Option<T>,
    T: Copy + std::fmt::Debug,
{
    use std::time::Instant;

    println!("Running day: {}", day);

    let now = Instant::now();
    if let Some(res) = part_one(input) {
        println!("Part one: {:?}", res);
        println!("Ran in: {:.2?}", now.elapsed());
    } else {
        println!("Part one did not run.");
    }

    let now = Instant::now();
    if let Some(res) = part_two(input) {
        println!("Part two: {:?}", res);
        println!("Ran in: {:.2?}", now.elapsed());
    } else {
        println!("Part two did not run.");
    }
}

pub fn create_files(day: Day) {
    let input_path = format!("./inputs/{day}.txt");
    let example_path = format!("./examples/{day}.txt");
    let module_path = format!("./src/bin/{day}.rs");

    let module_template =
        MODULE_TEMPLATE_TEXT.replace("%%DAY%%", format!("{:?}", day.into_inner()).as_str());
    let _ = fs::write(module_path, module_template);

    let downloaded_input = download_day(day).expect("day to be downloaded");
    let _ = fs::write(input_path, downloaded_input);

    let _ = fs::write(example_path, "");
    println!("--> REMEMBER TO FILL OUT EXAMPLE PATH <--");
}

const MODULE_TEMPLATE_TEXT: &str = r"advent_of_code_2024::solution!(%%DAY%%);

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

#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        $crate::solution!(@impl $day, [part_one, 1] [part_two, 2]);
    };

    (@impl $day:expr, $( [$func:expr, $part:expr] )*) => {
        /// The current day.
        const DAY: $crate::day::Day = $crate::day!($day);

        fn main() {
            let input = $crate::read_input(DAY);
            $crate::execute_parts(&input, DAY, part_one, part_two);
        }
    };
}
