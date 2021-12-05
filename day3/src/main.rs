use common::{run, Result};

fn zip(array: &Vec<String>) -> Vec<String> {
    let mut zipped: Vec<String> = vec![String::new(); array[0].len()];

    for s in array {
        for (i, v) in s.char_indices() {
            zipped[i].push(v.clone());
        }
    }

    zipped
}

fn part1(input: &str) -> Result<isize> {
    let lines: Vec<_> = input.lines().map(|s| s.to_owned()).collect();

    let gamma = zip(&lines)
        .iter()
        .map(|l| {
            let (ones, zeroes) = l.chars().fold((0, 0), |t, c| match c {
                '1' => (t.0 + 1, t.1),
                '0' => (t.0, t.1 + 1),
                _ => unreachable!(),
            });

            if ones > zeroes {
                "1"
            } else {
                "0"
            }
        })
        .collect::<Vec<_>>()
        .join("");

    let gamma = isize::from_str_radix(&gamma, 2)?;

    Ok(gamma * (gamma ^ 0b111111111111))
}

fn part2(_input: &str) -> Result<isize> {
    Ok(0)
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    run(input, &part1, &part2)?;

    Ok(())
}
