use std::collections::HashMap;

use itertools::Itertools;

use crate::util::{Frequencies, load};

type Elements = HashMap<char, u64>;
type Pairs = HashMap<(char, char), u64>;
type Rules = HashMap<(char, char), char>;

fn input() -> (Elements, Pairs, Rules) {
    let lines: Vec<String> = load("data/day14.txt");
    let elements = lines[0].chars().frequencies();
    let pairs = lines[0].chars().zip(lines[0].chars().skip(1))
        .map(|(c1, c2)| (c1, c2))
        .frequencies::<u64>();
    let mut rules = HashMap::new();
    for line in lines[2..].into_iter() {
        let c: Vec<char> = line.chars().collect();
        rules.insert((c[0], c[1]), c[6]);
    }
    (elements, pairs, rules)
}

fn evolve_once(elements: &mut Elements, pairs: Pairs, rules: &Rules) -> Pairs {
    let mut new_pairs: Pairs = HashMap::new();
    for ((a, c), cnt) in pairs {
        let b = rules[&(a, c)];
        *new_pairs.entry((a, b)).or_default() += cnt;
        *new_pairs.entry((b, c)).or_default() += cnt;
        *elements.entry(b).or_default() += cnt;
    }
    new_pairs
}

fn evolve(elements: &mut Elements, pairs: Pairs, rules: &Rules, generations: usize) -> Pairs {
    (0..generations).fold(pairs, |acc, _gen| evolve_once(elements, acc, rules))
}

fn delta(elements: Elements) -> u64 {
    let sorted: Vec<u64> = elements.into_values().sorted_unstable().collect();
    sorted[sorted.len() - 1] - sorted[0]
}

fn part1(mut elements: Elements, pairs: Pairs, rules: Rules) -> u64 {
    evolve(&mut elements, pairs, &rules, 10);
    delta(elements)
}


fn part2(mut elements: Elements, pairs: Pairs, rules: Rules) -> u64 {
    evolve(&mut elements, pairs, &rules, 40);
    delta(elements)
}

#[cfg(test)]
mod tests {
    use crate::day14::{input, part1, part2};

    #[test]
    fn test_part1() {
        let (elements, pairs, rules) = input();
        assert_eq!(part1(elements, pairs, rules), 2657);
    }

    #[test]
    fn test_part2() {
        let (elements, pairs, rules) = input();
        assert_eq!(part2(elements, pairs, rules), 2911561572630);
    }
}
