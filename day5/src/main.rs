use common::{run, Part, Result};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
struct Point(i32, i32);
impl FromIterator<i32> for Point {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut pt = iter.into_iter().take(2);

        Self(pt.next().unwrap(), pt.next().unwrap())
    }
}

fn f(m: i32, x1: i32, y1: i32, x: i32) -> i32 {
    let y = (m * (x - x1)) + y1;
    y
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
struct Line(Point, Point);
impl Line {
    fn get_points(&self, part: &Part) -> Vec<Point> {
        let (x2, x1, y2, y1) = (self.1 .0, self.0 .0, self.1 .1, self.0 .1);

        if x1 == x2 {
            let (y2, y1) = if y1 > y2 { (y1, y2) } else { (y2, y1) };
            (y1..=y2).map(|y| Point(x1, y)).collect()
        } else if y1 == y2 {
            let (x2, x1) = if x1 > x2 { (x1, x2) } else { (x2, x1) };
            (x1..=x2).map(|x| Point(x, y1)).collect()
        } else if *part == Part::Part2 {
            let m = (y2 - y1) / (x2 - x1);

            let (x2, x1) = if x1 > x2 { (x1, x2) } else { (x2, x1) };
            let mut pts = Vec::new();
            for x in x1..=x2 {
                let y = f(m as i32, self.0 .0 as i32, self.0 .1 as i32, x as i32);
                pts.push(Point(x, y));
            }
            pts
        } else {
            Vec::new()
        }
    }
}

impl FromIterator<Point> for Line {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let mut pt = iter.into_iter().take(2);

        Self(pt.next().unwrap(), pt.next().unwrap())
    }
}

fn solve(input: &str, part: &Part) -> Result<u32> {
    let lines = input
        .lines()
        .map(|l| {
            l.split("->")
                .map(str::trim)
                .map(|p| {
                    p.split(',')
                        .map(str::trim)
                        .map(|p| p.parse::<i32>().unwrap())
                        .collect::<Point>()
                })
                .collect::<Line>()
        })
        .filter(|&l| {
            if let Part::Part1 = *part {
                l.0 .0 == l.1 .0 || l.0 .1 == l.1 .1
            } else {
                true
            }
        })
        .collect::<Vec<_>>();

    let max_x = lines.iter().fold(0, |a, l| {
        let (x1, x2) = (l.0 .0, l.1 .0);
        if a < x1 {
            x1
        } else if a < x2 {
            x2
        } else {
            a
        }
    });
    let max_y = lines.iter().fold(0, |a, l| {
        let (y1, y2) = (l.0 .1, l.1 .1);
        if a < y1 {
            y1
        } else if a < y2 {
            y2
        } else {
            a
        }
    });

    let mut graph = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];

    lines.iter().for_each(|line| {
        line.get_points(part).iter().for_each(|p| {
            let (x, y) = (p.0, p.1);
            graph[y as usize][x as usize] += 1;
        })
    });

    let ans = graph.iter().flatten().filter(|&&p| p >= 2).count();

    Ok(ans as u32)
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
