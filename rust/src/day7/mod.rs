use crate::util::{abs_diff, Frequencies, load};

fn input() -> Vec<u32> {
    let lines: Vec<String> = load("data/day7.txt");
    lines[0]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn total_fuel<S, F>(positions: &mut [u32], start: S, fuel: F) -> u32
    where S: Fn(&mut [u32]) -> u32,
          F: Fn(u32, u32) -> u32,
{
    let pos = start(positions);
    positions.into_iter()
        .frequencies::<u32>()
        .iter()
        .fold(0, |acc, (p, cnt)| acc + cnt * fuel(pos, **p))
}

fn average(data: &mut [u32]) -> u32 {
    data.iter().sum::<u32>() / data.len() as u32
}

fn median(data: &mut [u32]) -> u32 {
    data.sort();
    data[data.len() / 2]
}

fn constant_fuel(from: u32, to: u32) -> u32 {
    abs_diff(from, to)
}

fn incremental_fuel(from: u32, to: u32) -> u32 {
    let delta = abs_diff(from, to);
    delta * (delta + 1) / 2
}

fn part1(positions: &mut [u32]) -> u32 {
    total_fuel(positions, median, constant_fuel)
}

fn part2(positions: &mut [u32]) -> u32 {
    total_fuel(positions, average, incremental_fuel)
}

#[cfg(test)]
mod tests {
    use crate::day7::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(&mut input()), 343468);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&mut input()), 96086265);
    }
}