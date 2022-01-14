use std::cmp::{max, min};
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::util::load;

type Coord = (i64, i64, i64);

struct Cuboid {
    add_sub: i64,
    c1: Coord,
    c2: Coord,
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn to_i64(cap: &Captures, i: usize) -> i64 {
            cap.get(i).unwrap().as_str().parse().unwrap()
        }

        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();
        }
        match RE.captures(s) {
            Some(cap) =>
                Ok(Cuboid {
                    add_sub: (if cap.get(1).unwrap().as_str() == "on" { 1 } else { -1 }),
                    c1: (to_i64(&cap, 2), to_i64(&cap, 4), to_i64(&cap, 6)),
                    c2: (to_i64(&cap, 3), to_i64(&cap, 5), to_i64(&cap, 7)),
                }),
            None => panic!(),
        }
    }
}

impl Cuboid {
    fn add_sub_invert(&mut self) {
        self.add_sub *= -1;
    }

    fn volume(&self) -> i64 {
        (self.c2.0 - self.c1.0 + 1).abs() *
            (self.c2.1 - self.c1.1 + 1).abs() *
            (self.c2.2 - self.c1.2 + 1).abs()
    }

    fn is_disjoint(&self, other: &Cuboid) -> bool {
        (self.c2.0 < other.c1.0 || other.c2.0 < self.c1.0) ||
            (self.c2.1 < other.c1.1 || other.c2.1 < self.c1.1) ||
            (self.c2.2 < other.c1.2 || other.c2.2 < self.c1.2)
    }

    fn intersection(&self, other: &Cuboid) -> Self {
        Cuboid {
            add_sub: self.add_sub * other.add_sub,
            c1: (max(self.c1.0, other.c1.0),
                 max(self.c1.1, other.c1.1),
                 max(self.c1.2, other.c1.2)),
            c2: (min(self.c2.0, other.c2.0),
                 min(self.c2.1, other.c2.1),
                 min(self.c2.2, other.c2.2)),
        }
    }
}

fn get_overlaps(done: &[Cuboid], other: &Cuboid) -> Vec<Cuboid> {
    let mut overlaps = Vec::new();
    for c in done {
        if !c.is_disjoint(other) {
            let mut overlap = c.intersection(other);
            if other.add_sub == 1 {
                // switch if new one is 'on'
                overlap.add_sub_invert();
            }
            overlaps.push(overlap);
        }
    }
    overlaps
}

fn solve(cuboids: Vec<Cuboid>) -> i64 {
    let mut done = Vec::new();
    for (i, c) in cuboids.into_iter().enumerate() {
        let mut overlaps = get_overlaps(&done, &c);
        done.append(&mut overlaps);
        if c.add_sub == 1 {
            done.push(c);
        }
        let s: i64 = done.iter()
            .map(|c| c.add_sub * c.volume())
            .sum();
        println!("{} {}", i, s)
    }
    done.into_iter()
        .map(|c| c.add_sub * c.volume())
        .sum()
}

fn input() -> Vec<Cuboid> {
    load("data/day22.txt")
}

fn part1(cuboids: Vec<Cuboid>) -> i64 {
    solve(cuboids.into_iter().take(20).collect())
}

fn part2(cuboids: Vec<Cuboid>) -> i64 {
    solve(cuboids)
}

#[cfg(test)]
mod tests {
    use crate::day22::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 650099);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1254011191104293);
    }
}
