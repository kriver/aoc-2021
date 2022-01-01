use std::cmp::max;
use std::collections::HashMap;
use std::num;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

use crate::util::load;

type Coord = (i32, i32);

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("")]
    Parse(#[from] num::ParseIntError),
    #[error("unmatched line")]
    Unmatched,
}

#[derive(Debug)]
struct Line {
    from: Coord,
    to: Coord,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        let (_x1, y1) = self.from;
        let (_x2, y2) = self.to;
        y1 == y2
    }

    fn is_vertical(&self) -> bool {
        let (x1, _y1) = self.from;
        let (x2, _y2) = self.to;
        x1 == x2
    }

    fn points(&self) -> Vec<Coord> {
        let (x1, y1) = self.from;
        let (x2, y2) = self.to;
        let (dx, dy) = (x2 - x1, y2 - y1);
        let steps = max(dx.abs(), dy.abs());
        (0..=steps)
            .map(|i| (x1 + i * dx / steps, y1 + i * dy / steps))
            .collect()
    }
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        }
        match RE.captures(s) {
            Some(cap) =>
                Ok(Line {
                    from: (cap.get(1).unwrap().as_str().parse::<i32>()?,
                           cap.get(2).unwrap().as_str().parse::<i32>()?),
                    to: (cap.get(3).unwrap().as_str().parse::<i32>()?,
                         cap.get(4).unwrap().as_str().parse::<i32>()?),
                }),
            None => Result::Err(ParseError::Unmatched),
        }
    }
}

fn input() -> Vec<Line> {
    let lines: Vec<Line> = load("data/day5.txt");
    lines
}

fn count_overlaps<F>(lines: Vec<Line>, f: F) -> usize
    where F: Fn(&Line) -> bool
{
    let mut counts: HashMap<Coord, u32> = HashMap::new();
    lines.iter()
        .filter(|l| f(*l))
        .flat_map(|l| l.points())
        .for_each(|c| *counts.entry(c).or_default() += 1);
    counts.values().filter(|c| **c > 1).count()
}

fn part1(lines: Vec<Line>) -> usize {
    count_overlaps(lines, |l| l.is_horizontal() || l.is_vertical())
}

fn part2(lines: Vec<Line>) -> usize {
    count_overlaps(lines, |_l| true)
}

#[cfg(test)]
mod tests {
    use crate::day5::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 4873);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 19472);
    }
}