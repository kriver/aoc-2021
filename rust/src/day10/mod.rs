use std::collections::HashMap;
use std::str::Chars;

use itertools::Itertools;
use lazy_static::lazy_static;

use crate::util::load;

fn input() -> Vec<String> {
    let lines: Vec<String> = load("data/day10.txt");
    lines
}

enum EatState {
    Corrupt(u64),
    Incomplete(u64),
    Ongoing(char),
}

impl EatState {
    fn score(&self) -> u64 {
        match self {
            EatState::Corrupt(score) => *score,
            EatState::Incomplete(score) => *score,
            _ => unreachable!("should not happen")
        }
    }

    fn corrupt(c: &char) -> EatState {
        lazy_static! {
            static ref CORRUPT: HashMap<char, u64> = HashMap::from(
                [(')', 3), (']', 57), ('}',1197), ('>', 25137)]);
        }
        EatState::Corrupt(CORRUPT[c])
    }

    fn incomplete(score: u64, c: &char) -> EatState {
        lazy_static! {
            static ref INCOMPLETE: HashMap<char, u64> = HashMap::from(
                [('(',1),( '[', 2), ('{', 3), ('<', 4)]);
        }
        EatState::Incomplete(score * 5 + INCOMPLETE[c])
    }
}

fn eat(chunk: &mut Chars) -> EatState {
    lazy_static! {
        static ref CLOSING: HashMap<char, char> = HashMap::from(
            [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    }
    match chunk.next() {
        Some(any) => match any {
            // found closing, return from recursion
            ')' | ']' | '}' | '>' => EatState::Ongoing(any),
            // found opening
            '(' | '[' | '{' | '<' => {
                match eat(chunk) {
                    EatState::Corrupt(score) => EatState::Corrupt(score),
                    EatState::Incomplete(score) => EatState::incomplete(score, &any),
                    EatState::Ongoing(closing) => match closing {
                        // valid closing for opening, so continue eating
                        ')' | ']' | '}' | '>' if closing == CLOSING[&any] => eat(chunk),
                        // invalid closing
                        _ => EatState::corrupt(&closing),
                    }
                }
            }
            _ => unreachable!("unexpected character")
        }
        // end of stream
        None => EatState::Incomplete(0)
    }
}

fn score(chunk: String) -> EatState {
    eat(&mut chunk.chars())
}

fn part1(lines: Vec<String>) -> u64 {
    lines.into_iter()
        .map(score)
        .filter(|eat| matches!(eat, EatState::Corrupt(_)))
        .map(|eat| eat.score())
        .sum()
}

fn part2(lines: Vec<String>) -> u64 {
    let scores: Vec<u64> = lines.into_iter()
        .map(score)
        .filter(|eat| matches!(eat, EatState::Incomplete(_)))
        .map(|eat| eat.score())
        .sorted_unstable()
        .collect();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use crate::day10::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 318099);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 2389738699);
    }
}
