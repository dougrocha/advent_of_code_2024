pub mod day;
pub mod template;

use day::Day;
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

#[derive(Default)]
pub struct Inputs {
    session_token: Option<String>,
}

impl Inputs {
    pub fn new() -> Self {
        Inputs::default()
    }

    pub fn get(&mut self, day: Day) -> Result<Vec<u8>> {
        let path = &format!("./inputs/{:0>2}.txt", day.into_inner());
        let path = Path::new(&path);

        if let Ok(input) = fs::read(path) {
            return Ok(input);
        }

        let input = self.download_day(day)?;
        let _ = fs::write(path, &input);
        Ok(input)
    }

    fn get_session_token(&mut self) -> Result<&str> {
        if self.session_token.is_none() {
            let session_token = fs::read_to_string("./session_token.txt")
                .expect("Should have been able to read token");
            self.session_token = Some(session_token.trim().to_string());
        }
        Ok(self.session_token.as_ref().unwrap())
    }

    pub fn download_day(&mut self, day: Day) -> Result<Vec<u8>> {
        let session_token = self.get_session_token()?;
        let cookie = format!("session={session_token}");

        let href = format!("https://adventofcode.com/2024/day/{:?}/input", day);
        let body = ureq::get(&href)
            .set("cookie", &cookie)
            .timeout(Duration::from_secs(3))
            .call()
            .map_err(Box::new)?;

        let mut buf = Vec::new();
        let _ = body.into_reader().read_to_end(&mut buf);

        Ok(buf)
    }
}

fn get_path(path: &str, day: Day) -> PathBuf {
    let path = &format!("./{path}/{:0>2}.txt", day.into_inner());
    let path = Path::new(path);

    path.to_path_buf()
}

fn get_session_token() -> Result<String> {
    Ok(fs::read_to_string("./session_token.txt")?)
}

fn download_day(day: Day) -> Result<String> {
    let session_token = get_session_token()?;
    let cookie = format!("session={session_token}");

    let href = format!("https://adventofcode.com/2024/day/{:?}/input", day);
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

pub fn execute_parts<F1, F2>(input: &str, day: Day, part_one: F1, part_two: F2)
where
    F1: FnOnce(&str) -> Option<u32>,
    F2: FnOnce(&str) -> Option<u32>,
{
    println!("Running day: {}", day);
    if let Some(res) = part_one(input) {
        println!("Part one: {:?}", res);
    }

    if let Some(res) = part_two(input) {
        println!("Part two: {:?}", res);
    }
}

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
