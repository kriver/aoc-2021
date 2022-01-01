use std::collections::HashMap;

use crate::util::{Frequencies, load};

type Freq = HashMap<u32, u64>;


fn input() -> Freq {
    let lines: Vec<String> = load("data/day6.txt");
    lines[0]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .frequencies::<u64>()
}

fn evolve_once(fish: Freq) -> Freq {
    let mut new_fish: Freq = HashMap::new();
    for (days, cnt) in fish.iter() {
        match days {
            0 => {
                new_fish.insert(8, *cnt);
                *new_fish.entry(6).or_default() += cnt;
            }
            _ => *new_fish.entry(days - 1).or_default() += cnt,
        }
    };
    new_fish
}

fn evolve(fish: Freq, days: u32) -> u64 {
    let mut f = fish;
    for _ in 0..days {
        f = evolve_once(f);
    }
    f.values().sum()
}

fn part1(fish: Freq) -> u64 {
    evolve(fish, 80)
}

fn part2(fish: Freq) -> u64 {
    evolve(fish, 256)
}

#[cfg(test)]
mod tests {
    use crate::day6::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 345387);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1574445493136);
    }
}