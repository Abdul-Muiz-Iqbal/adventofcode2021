use std::{env, num::ParseIntError, str::FromStr};

enum Part {
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

type Result<T> = std::result::Result<T, ParseIntError>;

fn part1(input: &str) -> Result<i32> {
    let ans = input
        .trim()
        .lines()
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<i32>>>()?
        .windows(2)
        .fold(0, |x, w| if w[0] - w[1] < 0 { x + 1 } else { x });

    Ok(ans)
}

fn part2(input: &str) -> Result<i32> {
    let ans = input
        .trim()
        .lines()
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<i32>>>()?
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<_>>()
        .windows(2)
        .fold(0, |x, w| if w[0] - w[1] < 0 { x + 1 } else { x });

    Ok(ans)
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");

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
