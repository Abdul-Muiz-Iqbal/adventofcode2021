#[derive(Debug, Clone, Copy)]
enum Cell {
    Marked(u32),
    Unmarked(u32),
}

impl Cell {
    fn value(&self) -> u32 {
        match *self {
            Self::Marked(v) => v,
            Self::Unmarked(v) => v,
        }
    }
}
/**
 * [[1, 2], [3, 4], [5, 6]] len(A) = 3, len(A[n]) = 2
 * [[1, 3, 5], [2, 4, 6]] len(A) = 2, len(A[n]) = 3
 */
fn zip(array: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut zipped: Vec<Vec<Cell>> = vec![vec![Cell::Unmarked(0); array.len()]; array[0].len()];

    for (i, s) in array.iter().enumerate() {
        for (j, v) in s.iter().enumerate() {
            zipped[i][j] = v.clone();
        }
    }

    zipped
}

fn main() {
    let mut lines = include_str!("input.txt").lines();

    let nums: Vec<u32> = lines
        .nth(0)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let boards = lines
        .filter_map(|line| {
            let line = line.trim();

            if line.is_empty() {
                None
            } else {
                Some(line)
            }
        })
        .collect::<Vec<_>>();
    let mut boards: Vec<_> = boards
        .chunks(5)
        .map(|chunk| {
            chunk
                .iter()
                .map(|row| {
                    row.split(' ')
                        .filter_map(|s| {
                            if s.is_empty() {
                                None
                            } else {
                                Some(Cell::Unmarked(s.trim().parse::<u32>().unwrap()))
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut b = boards.clone();
    for n in nums.iter() {
        for (i, board) in boards.iter().enumerate() {
            for (j, row) in board.iter().enumerate() {
                for (k, cell) in row.iter().enumerate() {
                    if cell.value() == *n {
                        b[i][j][k] = Cell::Marked(cell.value());
                        let row_matches = b[i]
                            .iter()
                            .filter(|&c| {
                                let mut t = c.clone();
                                t.retain(|&c| match c {
                                    Cell::Marked(_) => false,
                                    Cell::Unmarked(_) => true,
                                });
                                t.is_empty()
                            })
                            .next()
                            .is_some();
                        let col_matches = zip(&b[i])
                            .iter()
                            .filter(|&c| {
                                let mut t = c.clone();
                                t.retain(|&c| match c {
                                    Cell::Marked(_) => false,
                                    Cell::Unmarked(_) => true,
                                });
                                t.is_empty()
                            })
                            .next()
                            .is_some();
                        // check if board won.
                        if row_matches || col_matches {
                            let sum = b[i]
                                .iter()
                                .flatten()
                                .inspect(|c| println!("{:?}", c))
                                .filter(|&&c| match c {
                                    Cell::Marked(_) => false,
                                    Cell::Unmarked(_) => true,
                                })
                                .fold(0, |a, b| a + b.value());

                            dbg!(sum, n);
                            return;
                        }
                    }
                }
            }
        }
    }
}
