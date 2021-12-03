use common::{run, Part, Result};
use std::str::FromStr;

enum Instruction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let words: Vec<_> = s.split(' ').collect();
        let (command, value): (_, u32) = (words[0], words[1].parse().unwrap());
        let ins = match command {
            "forward" => Self::Forward(value),
            "up" => Self::Up(value),
            "down" => Self::Down(value),
            _ => unreachable!(),
        };

        Ok(ins)
    }
}

struct Submarine {
    x: u32,
    y: u32,
    aim: u32,
}

impl Submarine {
    fn new(x: u32, y: u32, aim: u32) -> Self {
        Self { x, y, aim }
    }

    fn execute(&mut self, ins: &Instruction, part: &Part) {
        if let Part::Part1 = *part {
            match *ins {
                Instruction::Forward(v) => self.x += v,
                Instruction::Up(v) => self.y -= v,
                Instruction::Down(v) => self.y += v,
            }
        } else {
            match *ins {
                Instruction::Forward(v) => {
                    self.x += v;
                    self.y += self.aim * v;
                }
                Instruction::Up(v) => self.aim -= v,
                Instruction::Down(v) => self.aim += v,
            }
        }
    }
}

fn solve(input: &str, part: &Part) -> Result<u32> {
    let mut sub = Submarine::new(0, 0, 0);

    input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .for_each(|ins| sub.execute(&ins, part));

    Ok(sub.x * sub.y)
}

fn part1(input: &str) -> Result<u32> {
    solve(input, &Part::Part1)
}

fn part2(input: &str) -> Result<u32> {
    solve(input, &Part::Part2)
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    run(input, &part1, &part2)?;

    Ok(())
}
