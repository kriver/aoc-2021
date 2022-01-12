use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::util::load;

type Vector3D = [i32; 3];
type Matrix3D = [Vector3D; 3];

const ROTATIONS: [Matrix3D; 24] = [
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[-1, 0, 0], [0, 0, -1], [-0, -1, -0]],
    [[-1, 0, 0], [0, 0, 1], [0, 1, -0]],
    [[1, 0, 0], [0, 0, -1], [-0, 1, 0]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    [[0, -1, 0], [-1, 0, 0], [-0, -0, -1]],
    [[0, -1, 0], [1, 0, 0], [-0, 0, 1]],
    [[0, 1, 0], [-1, 0, 0], [0, -0, 1]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, 0, -1], [0, -1, 0], [-1, -0, -0]],
    [[0, 0, -1], [0, 1, 0], [1, -0, 0]],
    [[0, 0, 1], [0, -1, 0], [1, 0, -0]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]]
];

#[derive(Debug, Eq, PartialEq, Hash)]
struct Beacon(Vector3D);

impl Beacon {
    fn rotate(&self, r: &Matrix3D) -> Beacon {
        let loc = self.0;
        Beacon(
            r.iter()
                .map(|row| row
                    .zip(loc)
                    .into_iter()
                    .map(|(a, b)| a * b)
                    .sum())
                .collect::<Vec<i32>>()
                .try_into().unwrap()
        )
    }

    fn distance_to(&self, other: &Beacon) -> Vector3D {
        let a = self.0;
        let b = other.0;
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }

    fn with_offset(&self, offset: &Vector3D) -> Beacon {
        let loc = self.0;
        Beacon([loc[0] + offset[0], loc[1] + offset[1], loc[2] + offset[2]])
    }
}

impl From<&str> for Beacon {
    fn from(line: &str) -> Self {
        let coords: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        Beacon([coords[0], coords[1], coords[2]])
    }
}

impl From<Vec<i32>> for Beacon {
    fn from(v: Vec<i32>) -> Self {
        Beacon([v[0], v[1], v[2]])
    }
}

#[derive(Debug)]
struct Scanner {
    id: u32,
    beacons: Vec<Beacon>,
    rotations: Vec<Vec<Beacon>>,
    rot: usize,
    offset: Vector3D,
}

impl Scanner {
    fn new(id: u32) -> Scanner {
        Scanner { id, beacons: vec![], rotations: vec![], rot: 0, offset: [0; 3] }
    }

    fn add(&mut self, beacon: Beacon) {
        self.beacons.push(beacon);
    }

    fn prepare_rotations(&mut self) {
        self.rotations = ROTATIONS.iter()
            .map(|r| self.beacons.iter()
                .map(|b| b.rotate(r))
                .collect())
            .collect()
    }

    fn overlaps(&self, other: &mut Scanner) -> bool {
        fn add(a: &Vector3D, b: &Vector3D) -> Vector3D {
            [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
        }

        for (i, r) in other.rotations.iter().enumerate() {
            let distance_counts = self.rotations[self.rot].iter()
                .cartesian_product(r.iter())
                .map(|(b1, b2)| b1.distance_to(b2))
                .counts();
            for (offset, count) in distance_counts.into_iter() {
                if count >= 12 {
                    other.rot = i;
                    other.offset = add(&self.offset, &offset);
                    return true;
                }
            }
        }
        false
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scanner(id:{}, rot:{}, offset:{:?}, #beacons: {})",
               self.id, self.rot, self.offset, self.beacons.len())
    }
}

impl From<&str> for Scanner {
    fn from(line: &str) -> Self {
        let id = line.split(" ").nth(2).unwrap().parse::<u32>().unwrap();
        Scanner::new(id)
    }
}

struct BeaconMap {
    beacons: HashSet<Beacon>,
}

impl BeaconMap {
    fn add(&mut self, scanner: &Scanner) {
        for b in &scanner.rotations[scanner.rot] {
            self.beacons.insert(b.with_offset(&scanner.offset));
        }
    }
}

fn input() -> Vec<Scanner> {
    type Accumulator = (Vec<Scanner>, Option<Scanner>);
    fn parse_line((mut scanners, mut scanner): Accumulator, line: String) -> Accumulator {
        if line.starts_with("---") {
            if let Some(s) = scanner {
                scanners.push(s);
            }
            scanner = Some(Scanner::from(line.as_str()))
        } else if !line.is_empty() {
            if let Some(ref mut s) = scanner {
                s.add(Beacon::from(line.as_str()));
            }
        }
        (scanners, scanner)
    }

    let lines: Vec<String> = load("data/day19.txt");
    let (mut scanners, scanner) = lines.into_iter()
        .fold((vec![], None), parse_line);
    if let Some(s) = scanner {
        scanners.push(s);
    }
    scanners
}

fn solve(scanners: &mut Vec<Scanner>) -> BeaconMap {
    scanners.iter_mut().for_each(|s| s.prepare_rotations());
    let mut unsolved = Vec::from_iter(scanners.iter_mut());
    let mut solved = vec![unsolved.remove(0)];
    let mut combined = BeaconMap { beacons: HashSet::new() };
    combined.add(solved[0]);
    while !unsolved.is_empty() {
        let current = solved.remove(0);
        unsolved = unsolved.into_iter().fold(
            vec![],
            |mut unsolved, u| {
                if current.overlaps(u) {
                    combined.add(u);
                    solved.push(u);
                } else {
                    unsolved.push(u);
                }
                unsolved
            });
    }
    combined
}

fn part1(mut scanners: Vec<Scanner>) -> usize {
    let map = solve(&mut scanners);
    map.beacons.len()
}


fn part2(mut scanners: Vec<Scanner>) -> i32 {
    fn manhattan_distance(a: &Vector3D, b: &Vector3D) -> i32 {
        (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
    }

    solve(&mut scanners);
    scanners.into_iter()
        .map(|s| s.offset)
        .combinations(2)
        .map(|v| manhattan_distance(&v[0], &v[1]))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day19::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 403);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 10569);
    }
}
