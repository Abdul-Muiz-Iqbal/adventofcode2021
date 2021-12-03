#![allow(dead_code)]

use std::{env, fmt::Debug, num::ParseIntError, str::FromStr};

pub enum Part {
    Part1,
    Part2,
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "part1" => Ok(Self::Part1),
            "part2" => Ok(Self::Part2),
            _ => Ok(Self::Part1),
        }
    }
}

pub type Result<T> = std::result::Result<T, ParseIntError>;
pub type Solution<'a, T> = &'a dyn Fn(&str) -> Result<T>;

pub fn run<T>(input: &str, part1: Solution<T>, part2: Solution<T>) -> Result<()>
where
    T: Debug,
{
    let part: Part = env::args()
        .nth(1)
        .map(|s| s.parse().unwrap())
        .or(Some(Part::Part1))
        .unwrap();
    let ans = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input),
    }?;

    dbg!(ans);

    Ok(())
}
