use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use itertools::Itertools;

use crate::util::load;

type Pattern = HashSet<char>;
type Decoded = HashMap<u8, Pattern>;

#[derive(Debug)]
struct Display {
    input: Vec<Pattern>,
    output: Vec<Pattern>,
}

fn parse_io(s: &str) -> Vec<Pattern> {
    s.split_whitespace()
        .map(|s| HashSet::from_iter(s.chars())) // drop empty last
        .collect()
}

impl FromStr for Display {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let io: Vec<&str> = s.split(" | ").collect();
        Ok(Display {
            input: parse_io(io[0]),
            output: parse_io(io[1]),
        })
    }
}

fn input() -> Vec<Display> {
    let displays: Vec<Display> = load("data/day8.txt");
    displays
}

fn decode_one(pattern: &Pattern, decoded: &Decoded) -> Option<u8> {
    match pattern.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        5 if decoded.contains_key(&1)
            && decoded[&1].intersection(pattern).count() == 2 => Some(3),
        5 if decoded.contains_key(&3) && decoded.contains_key(&4) =>
            match decoded[&4].intersection(pattern).count() {
                2 => Some(2),
                3 => Some(5),
                _ => None,
            }
        6 if decoded.contains_key(&7)
            && decoded[&7].intersection(pattern).count() == 2 => Some(6),
        6 if decoded.contains_key(&5)
            && decoded[&5].intersection(pattern).count() == 5 => Some(9),
        6 if decoded.contains_key(&6)
            && decoded.contains_key(&9) => Some(0),
        _ => None
    }
}

fn decode(display: Display) -> u32 {
    let mut decoded: Decoded = HashMap::new();
    let mut encoded = display.input;
    // step 1
    while encoded.len() > 0 {
        let mut new_encoded = Vec::new();
        encoded.into_iter()
            .for_each(|pattern|
                match decode_one(&pattern, &decoded) {
                    Some(n) => { decoded.insert(n, pattern); }
                    None => new_encoded.push(pattern),
                });
        encoded = new_encoded;
    }
    // step 2
    let inverted: HashMap<String, u32> = HashMap::from_iter(
        decoded
            .into_iter()
            .map(|(k, v)| (v.into_iter().sorted().collect::<String>(), k as u32))
    );
    display.output.into_iter()
        .fold(0, |acc, pattern| {
            let s = pattern.into_iter().sorted().collect::<String>();
            acc * 10 + inverted[&s]
        })
}

fn part1(displays: Vec<Display>) -> u32 {
    displays.into_iter()
        .flat_map(|d| d.output)
        .fold(0, |acc, s| match s.len() {
            2 | 3 | 4 | 7 => acc + 1,
            _ => acc
        })
}

fn part2(displays: Vec<Display>) -> u32 {
    displays.into_iter()
        .map(|d| decode(d))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day8::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 310);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 915941);
    }
}