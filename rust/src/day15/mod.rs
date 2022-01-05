use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};

use crate::util::load;

type Grid = Vec<Vec<u32>>;
type Coord = (i32, i32);

const DIM: i32 = 100;
const NEIGHBOURS: [Coord; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn input() -> Grid {
    let lines: Vec<String> = load("data/day15.txt");
    lines.into_iter()
        .map(|line| line.chars()
            .map(|c| c as u32 - '0' as u32)
            .collect())
        .collect()
}

#[derive(Eq, Debug)]
struct Location {
    risk: u32,
    pos: Coord,
}

impl Location {
    fn dist(&self) -> i32 {
        let (x, y) = self.pos;
        x + y
    }
}

impl PartialEq<Self> for Location {
    fn eq(&self, other: &Self) -> bool {
        self.risk == other.risk && self.pos == other.pos
    }
}

impl PartialOrd<Self> for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.risk.cmp(&other.risk) {
            Ordering::Equal => match self.dist().cmp(&other.dist()) {
                Ordering::Equal => Ordering::Less, // don't care
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less, // ensure bigger is better
            },
            other => other,
        }
    }
}

fn find_path_r(mut queue: BTreeSet<Location>, mut visited: HashSet<Coord>,
               grid: Grid, dest: Coord) -> u32 {
    while !queue.is_empty() {
        match queue.pop_first().unwrap() {
            Location { risk, pos } if dest == pos => return risk,
            Location { risk, pos } => {
                for (dx, dy) in NEIGHBOURS.iter() {
                    let (x, y) = pos;
                    let new_pos = (x + *dx, y + *dy);
                    let (nx, ny) = new_pos;
                    if nx < 0 || nx >= DIM || ny < 0 || ny >= DIM || visited.contains(&new_pos) {
                        continue;
                    }
                    let new_risk = risk + grid[ny as usize][nx as usize];
                    queue.insert(Location { risk: new_risk, pos: new_pos });
                    visited.insert(new_pos);
                }
            }
        }
    }
    unreachable!("no path found?")
}

fn find_path(grid: Grid, start: Coord, dest: Coord) -> u32 {
    let visited = HashSet::from([start]);
    let queue = BTreeSet::from([Location { risk: 0, pos: start }]);
    find_path_r(queue, visited, grid, dest)
}

fn part1(grid: Grid) -> u32 {
    find_path(grid, (0, 0), (DIM - 1, DIM - 1))
}


fn part2(grid: Grid) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day15::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 373);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 2868);
    }
}
