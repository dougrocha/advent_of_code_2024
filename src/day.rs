use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub struct Day(u8);

impl Day {
    pub fn new(day: u8) -> Self {
        assert!(
            (0..25).contains(&day),
            "day is not valid `{day}`, should be between 1 and 25"
        );

        Day(day)
    }

    pub fn into_inner(self) -> u8 {
        self.0
    }

    pub const fn __new_unchecked(day: u8) -> Self {
        Day(day)
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl FromStr for Day {
    type Err = super::Error;

    fn from_str(s: &str) -> super::Result<Self> {
        let day = u8::from_str(s).expect("string to be day format");

        Ok(Day(day))
    }
}

#[macro_export]
macro_rules! day {
    ($day:expr) => {{
        const _ASSERT: () = assert!(
            $day != 0 && $day <= 25,
            concat!(
                "invalid day number `",
                $day,
                "`, expecting a value between 1 and 25"
            ),
        );
        $crate::day::Day::__new_unchecked($day)
    }};
}
