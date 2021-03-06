use std::collections::{HashMap, VecDeque};

use crate::day12::RecursionType::{OneTwice, OnlyOnce};
use crate::util::load;

#[derive(Eq, PartialEq, Hash)]
struct Cave {
    id: String,
}

enum RecursionType {
    OnlyOnce,
    OneTwice,
}

impl Cave {
    fn new(id: String) -> Cave {
        Cave {
            id
        }
    }

    fn is_start(&self) -> bool {
        self.id == "start"
    }

    fn is_end(&self) -> bool {
        self.id == "end"
    }

    fn is_small(&self) -> bool {
        self.id.chars().all(|c| c.is_ascii_lowercase())
    }
}

struct Caves {
    caves: HashMap<Cave, Vec<Cave>>,
}

impl Caves {
    fn new() -> Caves {
        Caves {
            caves: HashMap::new(),
        }
    }

    fn add(&mut self, id: &str) {
        let cave = Cave::new(id.to_string());
        if !self.caves.contains_key(&cave) {
            self.caves.insert(cave, Vec::new());
        }
    }

    fn add_connection(&mut self, id_a: &str, id_b: &str) {
        let a = Cave::new(id_a.to_string());
        let b = Cave::new(id_b.to_string());
        if !a.is_start() {
            self.caves.get_mut(&b).unwrap().push(a);
        }
    }

    fn is_allowed(&self, recursion_type: &RecursionType, cave: &Cave, path: &mut VecDeque<&Cave>)
                  -> (RecursionType, bool) {
        match recursion_type {
            OnlyOnce => (OnlyOnce, !cave.is_small() || !path.contains(&cave)),
            OneTwice => (
                if cave.is_small() && path.contains(&cave) { OnlyOnce } else { OneTwice },
                true
            )
        }
    }

    fn walk<'a>(&'a self, cave: &'a Cave, path: &mut VecDeque<&'a Cave>, count: &mut usize,
                recursion_type: &RecursionType) {
        for c in self.caves[cave].iter() {
            match c.is_end() {
                true => *count += 1,
                false => {
                    let (new_rt, allowed) = self.is_allowed(recursion_type, c, path);
                    if allowed {
                        path.push_front(c);
                        self.walk(c, path, count, &new_rt);
                        path.pop_front();
                    }
                }
            }
        }
    }

    fn count_paths(&self, recursion_type: &RecursionType) -> usize
    {
        let mut paths = 0;
        let start = self.caves.keys().filter(|c| c.is_start()).nth(0).unwrap();
        self.walk(start, &mut VecDeque::new(), &mut paths, recursion_type);
        paths
    }
}

fn input() -> Caves {
    let lines: Vec<String> = load("data/day12.txt");
    let mut caves = Caves::new();
    for line in lines {
        let tokens: Vec<&str> = line.split("-").collect();
        let a = tokens[0];
        let b = tokens[1];
        caves.add(&a);
        caves.add(&b);
        caves.add_connection(&a, &b);
        caves.add_connection(&b, &a);
    }
    caves
}

fn part1(caves: &Caves) -> usize {
    caves.count_paths(&OnlyOnce)
}

fn part2(caves: &Caves) -> usize {
    caves.count_paths(&OneTwice)
}

#[cfg(test)]
mod tests {
    use crate::day12::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 5076);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 145643);
    }
}
