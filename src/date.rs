use std::{fmt::Display, str::FromStr};

use crate::err::Error;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Date {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl FromStr for Date {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c: Vec<_> = s.split('-').collect();
        match c[..] {
            [ y, m, d ] => {
                Ok(Self::new(y.parse()?, m.parse()?, d.parse()?))
            }
            [ y ] => Ok(Self::year(y.parse()?)),
            _ => Err(Error::ParseDate),
        }
    }
}

impl Date {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self {
            year,
            month,
            day,
        }
    }

    pub fn year(year: i32) -> Self {
        Self {
            year,
            month: 1,
            day: 1,
        }
    }
}
