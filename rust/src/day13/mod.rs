use std::collections::HashSet;
use std::iter::repeat;

use itertools::Itertools;

use crate::util::load;

const FOLD: &str = "fold along ";

type Dot = (usize, usize);
type Instr = (char, usize);

fn input() -> (HashSet<Dot>, Vec<Instr>) {
    let lines: Vec<String> = load("data/day13.txt");
    let mut dots = HashSet::new();
    let mut instr = Vec::new();
    for line in lines.into_iter() {
        if line.starts_with(FOLD) {
            let mut s = line[FOLD.len()..].split("=");
            instr.push((
                s.next().unwrap().chars().nth(0).unwrap(),
                s.next().unwrap().parse::<usize>().unwrap()))
        } else if !line.is_empty() {
            dots.insert(line.split(",").into_iter()
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple().unwrap());
        }
    }
    (dots, instr)
}

fn fold_once(dots: HashSet<Dot>, (dir, fold_at): (char, usize)) -> HashSet<Dot> {
    match dir {
        'x' => HashSet::from_iter(
            dots.into_iter()
                .map(|(x, y)| (if x > fold_at { 2 * fold_at - x } else { x }, y))
        ),
        'y' => HashSet::from_iter(
            dots.into_iter()
                .map(|(x, y)| (x, if y > fold_at { 2 * fold_at - y } else { y }))
        ),
        _ => unreachable!("should not happen")
    }
}

fn fold(dots: HashSet<Dot>, instr: &[Instr]) -> HashSet<Dot> {
    instr.into_iter()
        .fold(dots, |acc, i| fold_once(acc, *i))
}

fn part1(dots: HashSet<Dot>, instr: Vec<Instr>) -> usize {
    let dots = fold(dots, &instr[..1]);
    dots.len()
}

fn display(dots: &HashSet<Dot>) {
    let max_x = dots.iter().map(|(x, _y)| *x).max().unwrap();
    let max_y = dots.iter().map(|(_x, y)| *y).max().unwrap();
    let mut grid: Vec<String> = repeat(".".repeat(max_x + 1)).take(max_y + 1).collect();
    for (x, y) in dots.iter() {
        grid[*y].replace_range(*x..=*x, "#");
    }
    grid.into_iter()
        .for_each(|line| println!("{}", line));
}

fn part2(dots: HashSet<Dot>, instr: Vec<Instr>) -> usize {
    let dots = fold(dots, &instr);
    display(&dots);
    dots.len()
}

#[cfg(test)]
mod tests {
    use crate::day13::{input, part1, part2};

    #[test]
    fn test_part1() {
        let (dots, instr) = input();
        assert_eq!(part1(dots, instr), 669);
    }

    #[test]
    fn test_part2() {
        let (dots, instr) = input();
        assert_eq!(part2(dots, instr), 90);
    }
}
