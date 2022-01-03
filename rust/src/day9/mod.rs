use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::util::load;

type Coord = (i8, i8);
type HeightMap = HashMap<Coord, u32>;

const NEIGHBOURS: [Coord; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn input() -> HeightMap {
    let lines: Vec<String> = load("data/day9.txt");
    let mut hm: HeightMap = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            hm.insert((x as i8, y as i8), c as u32 - '0' as u32);
        }
    }
    hm
}

fn find_minima(hm: &HeightMap) -> Vec<Coord> {
    let mut minima = Vec::new();
    for ((x, y), height) in hm.iter() {
        let cnt = NEIGHBOURS.iter()
            .fold(0, |acc, (dx, dy)| {
                let key = (x + dx, y + dy);
                if !hm.contains_key(&key) || hm[&key] > *height {
                    acc + 1
                } else {
                    acc
                }
            });
        if cnt == 4 {
            minima.push((*x, *y));
        }
    }
    minima
}

fn build_basin_recurse(hm: &HeightMap, basin: &mut HashSet<Coord>, c: &Coord) {
    if hm.contains_key(c) && !basin.contains(c) {
        if hm[c] < 9 {
            basin.insert(*c);
            let (x, y) = c;
            for (dx, dy) in NEIGHBOURS {
                build_basin_recurse(hm, basin, &(x + dx, y + dy));
            }
        }
    }
}

fn build_basin_one(hm: &HeightMap, c: &Coord) -> HashSet<Coord> {
    let mut basin = HashSet::new();
    build_basin_recurse(hm, &mut basin, c);
    basin
}

fn build_basins(hm: &HeightMap) -> Vec<HashSet<Coord>> {
    find_minima(hm).into_iter()
        .map(|c| build_basin_one(hm, &c))
        .collect()
}

fn part1(hm: HeightMap) -> u32 {
    find_minima(&hm).iter()
        .map(|coord| hm[coord] + 1)
        .sum()
}

fn part2(hm: HeightMap) -> usize {
    build_basins(&hm).into_iter()
        .map(|v| v.len())
        .sorted_unstable()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod tests {
    use crate::day9::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1045660);
    }
}