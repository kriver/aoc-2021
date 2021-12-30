use std::num::ParseIntError;
use std::str::FromStr;

use crate::util::load;

struct Command {
    direction: String,
    distance: u32,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(" ").collect();
        Ok(Command {
            direction: tokens[0].to_string(),
            distance: tokens[1].parse::<u32>()?,
        })
    }
}

fn input() -> Vec<Command> {
    let commands: Vec<Command> = load("data/day2.txt");
    commands
}

fn part1(commands: Vec<Command>) -> u32 {
    let mut pos = 0;
    let mut depth = 0;
    for c in commands.iter() {
        match c.direction.as_str() {
            "forward" => pos += c.distance,
            "down" => depth += c.distance,
            "up" => depth -= c.distance,
            _ => ()
        }
    }
    pos * depth
}

fn part2(commands: Vec<Command>) -> u32 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for c in commands.iter() {
        match c.direction.as_str() {
            "forward" => {
                pos += c.distance;
                depth += aim * c.distance;
            }
            "down" => aim += c.distance,
            "up" => aim -= c.distance,
            _ => ()
        }
    }
    pos * depth
}

#[cfg(test)]
mod tests {
    use crate::day2::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1561344);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1848454425);
    }
}