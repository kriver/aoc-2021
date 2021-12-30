use crate::util::load;

const ONE: u8 = '1' as u8;
const ZERO: u8 = '0' as u8;

fn input() -> Vec<String> {
    let binaries: Vec<String> = load("data/day3.txt");
    binaries
}

fn count_ones(binaries: &Vec<String>) -> Vec<u32> {
    let bits = binaries[0].len();
    let mut counts = vec![0; bits];
    for bin in binaries.iter() {
        for (i, c) in bin.chars().enumerate() {
            if c == '1' {
                counts[i] += 1
            }
        }
    }
    counts
}

fn part1(binaries: Vec<String>) -> u32 {
    let lines: u32 = binaries.len() as u32;
    let counts = count_ones(&binaries);
    let mut gamma = 0;
    let mut epsilon = 0;
    for c in counts {
        gamma = gamma * 2 + (if c > lines / 2 { 1 } else { 0 });
        epsilon = epsilon * 2 + (if c < lines / 2 { 1 } else { 0 });
    }
    gamma * epsilon
}

fn bit_criteria_filtering<F>(binaries: &Vec<String>, keep_one: F) -> u32
    where F: Fn(usize, usize) -> bool
{
    let mut kept: Vec<bool> = vec![true; binaries.len()];
    let mut num_kept = kept.len();
    let mut bit: usize = 0;
    kept = loop {
        let ones = binaries.iter().enumerate()
            .filter(|(i, bin)| kept[*i] && (*bin).as_bytes()[bit] == ONE)
            .count();
        let keep_ones = keep_one(ones, num_kept / 2);
        let keep_bit = if keep_ones { ONE } else { ZERO };
        kept = binaries.iter().enumerate()
            .map(|(i, bin)| kept[i] && bin.as_bytes()[bit] == keep_bit)
            .collect();
        num_kept = if keep_ones { ones } else { num_kept - ones };
        if num_kept == 1 {
            break kept;
        }
        bit += 1;
    };
    let idx = kept.iter().enumerate()
        .filter(|(_, v)| **v)
        .map(|(i, _)| i)
        .nth(0).unwrap();
    u32::from_str_radix(&binaries[idx], 2).unwrap()
}

fn part2(binaries: Vec<String>) -> u32 {
    let oxygen_rating = bit_criteria_filtering(&binaries, |a, b| a >= b);
    let co2_scrubber_rating = bit_criteria_filtering(&binaries, |a, b| a < b);
    oxygen_rating * co2_scrubber_rating
}

#[cfg(test)]
mod tests {
    use crate::day3::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 4118544);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 3832770);
    }
}