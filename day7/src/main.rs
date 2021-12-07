use common::{run, Part, Result};

fn solve(input: &str, part: &Part) -> Result<i32> {
    let positions = input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    let max = positions.iter().fold(0, |a, &b| if a < b { b } else { a });

    let ans = (0..(max + 2))
        .fold((0, i32::MAX), |best_pos, pos| {
            let total_fuel = positions.iter().fold(0, |total_fuel, &position| {
                if let Part::Part1 = *part {
                    total_fuel + (position - pos).abs()
                } else {
                    let n = (position - pos).abs();
                    total_fuel + ((((n * (n + 1)) as f64) / 2.0).round() as i32)
                }
            });
            if total_fuel < best_pos.1 {
                (pos, total_fuel)
            } else {
                best_pos
            }
        })
        .1;

    Ok(ans)
}

fn part1(input: &str) -> Result<i32> {
    solve(input, &Part::Part1)
}

fn part2(input: &str) -> Result<i32> {
    solve(input, &Part::Part2)
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    run(input, &part1, &part2)?;

    Ok(())
}
