use common::{run, Part, Result};
use std::collections::HashMap;

fn solve(input: &str, part: &Part) -> Result<u64> {
    let mut hm: HashMap<u64, u64> = HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
        (9, 0),
    ]);

    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .for_each(|num: u64| *hm.get_mut(&num).unwrap() += 1);

    let days = if let Part::Part1 = *part { 80 } else { 256 };

    (0..days).for_each(|_| {
        (0..9).for_each(|key| {
            let prev = if key == 0 { 9 } else { key - 1 };
            *hm.get_mut(&prev).unwrap() = hm[&key];
        });
        *hm.get_mut(&8).unwrap() = hm[&9];
        *hm.get_mut(&6).unwrap() += hm[&9];
        *hm.get_mut(&9).unwrap() = 0;
    });

    Ok(hm.values().sum())
}

fn part1(input: &str) -> Result<u64> {
    solve(input, &Part::Part1)
}

fn part2(input: &str) -> Result<u64> {
    solve(input, &Part::Part2)
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    run(input, &part1, &part2)?;

    Ok(())
}
