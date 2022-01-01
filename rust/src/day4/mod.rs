use std::collections::HashMap;

use retain_mut::RetainMut;

use crate::util::load;

type Moves = Vec<u32>;
type Coord = (usize, usize);

const DIM: usize = 5;

#[derive(Debug)]
struct BingoBoard {
    board: HashMap<u32, Coord>,
    rows: Vec<u32>,
    columns: Vec<u32>,
}

impl BingoBoard {
    fn empty() -> BingoBoard {
        BingoBoard {
            board: HashMap::new(),
            rows: vec![0; DIM],
            columns: vec![0; DIM],
        }
    }

    fn add(&mut self, x: usize, y: usize, n: u32) {
        self.board.insert(n, (x, y));
    }

    /// Checks for the presence of a drawn number and returns true if this makes
    /// the board win, false otherwise.
    fn draw(&mut self, n: u32) -> bool {
        match self.board.remove(&n) {
            Some((x, y)) => {
                // self.board.remove(&n);
                self.rows[y] += 1;
                self.columns[x] += 1;
                self.rows[y] == 5 || self.columns[x] == 5
            }
            None => false
        }
    }

    fn score(&self, n: u32) -> u32 {
        self.board.keys().sum::<u32>() * n
    }
}

fn input() -> (Moves, Vec<BingoBoard>) {
    let lines: Vec<String> = load("data/day4.txt");
    let moves = lines[0].split(",").map(|s| s.parse().unwrap()).collect();
    let mut boards = Vec::new();
    let mut board = BingoBoard::empty();
    let mut y = 0;
    for line in lines[2..].iter() {
        if line.len() == 0 {
            boards.push(board);
            board = BingoBoard::empty();
            y = 0;
        } else {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .enumerate()
                .for_each(|(x, n)| { board.add(x, y, n); });
            y += 1;
        }
    }
    boards.push(board);
    (moves, boards)
}

fn part1(moves: Moves, mut boards: Vec<BingoBoard>) -> u32 {
    for m in moves {
        for b in &mut boards {
            if b.draw(m) {
                return b.score(m);
            }
        }
    }
    unreachable!("no solution found")
}

fn part2(moves: Moves, mut boards: Vec<BingoBoard>) -> u32 {
    for m in moves {
        match boards.len() {
            1 => if boards[0].draw(m) {
                return boards[0].score(m);
            },
            _ => boards.retain_mut(|b| !b.draw(m))
        }
    }
    unreachable!("no solution found")
}

#[cfg(test)]
mod tests {
    use crate::day4::{input, part1, part2};

    #[test]
    fn test_part1() {
        let (moves, boards) = input();
        assert_eq!(part1(moves, boards), 63424);
    }

    #[test]
    fn test_part2() {
        let (moves, boards) = input();
        assert_eq!(part2(moves, boards), 23541);
    }
}
