use common::{run, Result};

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
    run(include_str!("input.txt"), &part1, &part2)?;

    Ok(())
}
