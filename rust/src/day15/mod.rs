use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};

use crate::util::load;

type Grid = Vec<Vec<u32>>;

static DIM: u32 = 100;
const NEIGHBOURS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn input() -> Grid {
    let lines: Vec<String> = load("data/day15.txt");
    lines.into_iter()
        .map(|line| line.chars()
            .map(|c| c as u32 - '0' as u32)
            .collect())
        .collect()
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Coord {
    x: u32,
    y: u32,
    // meta dimension
    mdim: u32,
    // meta coords
    mx: u32,
    my: u32,
}

impl Coord {
    fn dist(&self) -> u32 {
        (self.mx * self.mdim + self.x) + (self.my * self.mdim + self.y)
    }

    fn meta_dist(&self) -> u32 {
        self.mx + self.my
    }

    fn with_delta(&self, dx: i32, dy: i32) -> Option<Coord> {
        let (x, y) = (
            (self.mx * DIM + self.x) as i32 + dx,
            (self.my * DIM + self.y) as i32 + dy
        );
        if x < 0 || x as u32 >= DIM * self.mdim || y < 0 || y as u32 >= DIM * self.mdim {
            None
        } else {
            Some(Coord {
                x: x as u32 % DIM,
                y: y as u32 % DIM,
                mdim: self.mdim,
                mx: x as u32 / DIM,
                my: y as u32 / DIM,
            })
        }
    }
}

impl PartialOrd<Self> for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.dist().cmp(&other.dist()) {
            Ordering::Equal => Ordering::Less, // don't care
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less, // ensure bigger is better
        }
    }
}

#[derive(Eq, Debug)]
struct Location {
    risk: u32,
    pos: Coord,
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
            Ordering::Equal => self.pos.cmp(&other.pos),
            other => other,
        }
    }
}

fn grid_risk(grid: &Grid, pos: &Coord) -> u32 {
    let base_risk = grid[pos.y as usize][pos.x as usize];
    (base_risk + pos.meta_dist() - 1) % 9 + 1
}

fn find_path_r(mut queue: BTreeSet<Location>, mut visited: HashSet<Coord>,
               grid: Grid, dest: Coord) -> u32 {
    while !queue.is_empty() {
        match queue.pop_first().unwrap() {
            Location { risk, pos } if dest == pos => return risk,
            Location { risk, pos } => {
                for (dx, dy) in NEIGHBOURS.iter() {
                    match pos.with_delta(*dx, *dy) {
                        None => (), // off grid
                        Some(new_pos) => if !visited.contains(&new_pos) {
                            let new_risk = risk + grid_risk(&grid, &new_pos);
                            visited.insert(new_pos);
                            queue.insert(Location { risk: new_risk, pos: new_pos });
                        }
                    }
                }
            }
        }
    }
    unreachable!("no path found?")
}

fn find_path(grid: Grid, dest: Coord) -> u32 {
    let start = Coord { x: 0, y: 0, mdim: dest.mdim, mx: 0, my: 0 };
    let visited = HashSet::from([start]);
    let queue = BTreeSet::from([Location { risk: 0, pos: start }]);
    find_path_r(queue, visited, grid, dest)
}

fn part1(grid: Grid) -> u32 {
    let dest = Coord { x: DIM - 1, y: DIM - 1, mdim: 1, mx: 0, my: 0 };
    find_path(grid, dest)
}


fn part2(grid: Grid) -> u32 {
    let dest = Coord { x: DIM - 1, y: DIM - 1, mdim: 5, mx: 4, my: 4 };
    find_path(grid, dest)
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
