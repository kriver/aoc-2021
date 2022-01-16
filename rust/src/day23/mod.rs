use std::cmp::{max, min};
use std::collections::HashMap;
use std::mem::replace;

use lazy_static::lazy_static;

type C1 = u8;
type Coord = (C1, C1);

lazy_static! {
    static ref ENERGY: HashMap<char, u32> =
        HashMap::from([('A', 1), ('B', 10), ('C', 100), ('D', 1000)]);
}

#[derive(Clone)]
struct Amphipod {
    name: char,
    pos: Coord,
}

impl Amphipod {
    fn x(&self) -> C1 {
        self.pos.0
    }

    fn y(&self) -> C1 {
        self.pos.1
    }
    // Rooms are at x : 2, 4, 6, 8
    fn room_x(&self) -> C1 {
        2 * (self.name as C1 - 'A' as C1 + 1)
    }

    fn in_own_room(&self) -> bool {
        self.x() == self.room_x()
    }

    fn in_corridor(&self) -> bool {
        self.y() == 0
    }

    fn move_to(&mut self, dst: &Coord) -> u32 {
        let dist = self.x().abs_diff(dst.0) + self.y().abs_diff(dst.1);
        self.pos = *dst;
        dist as u32 * ENERGY[&self.name]
    }

    fn is_friend(&self, other: &Amphipod) -> bool {
        self.name == other.name
    }

    fn needs_to_move(&self, floor: &Floor) -> bool {
        if !self.in_own_room() {
            true // definitely need to move
        } else {
            // either we're at back of the room, or we're at the front and
            // our friends are already at the back
            for y in self.y() + 1..=floor.room_size() as u8 {
                match floor.get_at(self.x(), y) {
                    Some(a) if !self.is_friend(a) => return true,
                    _ => (),
                }
            }
            false
        }
    }

    fn into_room_moves(&self, floor: &Floor) -> Vec<Coord> {
        let min_x = min(self.x(), self.room_x());
        let max_x = max(self.x(), self.room_x());
        // hall free towards our room?
        let hall_free = (min_x..=max_x)
            .map(|x| floor.get_at(x, 0))
            .all(|o| o.is_none());
        let mut pos = vec![];
        if hall_free {
            // room contains our friends at the back and no-one else
            for y in (1..=floor.room_size()).rev() {
                match floor.get_at(self.room_x(), y as C1) {
                    None => {
                        pos.push((self.room_x(), y as C1));
                        break;
                    }
                    Some(a) if !self.is_friend(&a) => {
                        break;
                    }
                    _ => (),
                }
            }
        }
        pos
    }

    fn out_of_room_moves(&self, floor: &Floor) -> Vec<Coord> {
        // no-one in front of us to exit room?
        let can_exit = (1..self.y())
            .map(|y| floor.get_at(self.x(), y))
            .all(|o| o.is_none());
        if !can_exit {
            vec![] // no moves possible
        } else {
            let mut moves = Vec::new();
            for x in (0..self.x()).rev() {
                if x == 0 || x % 2 == 1 {
                    match floor.get_at(x, 0) {
                        Some(_) => break,
                        None => moves.push((x, 0)),
                    }
                }
            }
            for x in self.x() + 1..11 {
                if x == 10 || x % 2 == 1 {
                    match floor.get_at(x, 0) {
                        Some(_) => break,
                        None => moves.push((x, 0)),
                    }
                }
            }
            moves
        }
    }

    fn possible_moves(&self, floor: &Floor) -> Vec<Coord> {
        if self.in_corridor() {
            self.into_room_moves(floor)
        } else {
            self.out_of_room_moves(floor)
        }
    }
}

struct Floor {
    map: Vec<Vec<Option<Amphipod>>>,
}

impl Floor {
    fn new(amphipods: Vec<Amphipod>, room_size: usize) -> Floor {
        let mut map = vec![vec![None; 11]; room_size + 1];
        for a in amphipods.into_iter() {
            let (x, y) = a.pos;
            map[y as usize][x as usize] = Some(a);
        }
        Floor { map }
    }

    fn room_size(&self) -> usize {
        self.map.len() - 1
    }

    fn get_at(&self, x: C1, y: C1) -> &Option<Amphipod> {
        &self.map[y as usize][x as usize]
    }

    fn positions(&self) -> Vec<Coord> {
        let mut pos = vec![];
        for r in self.map.iter() {
            for a in r.iter() {
                match a {
                    Some(a) => pos.push(a.pos),
                    _ => (),
                }
            }
        }
        pos
    }

    fn move_out(&mut self, dst: &Coord) -> Amphipod {
        let (x, y) = dst;
        replace(&mut self.map[*y as usize][*x as usize], None).expect("should contain amphipod")
    }

    fn move_in(&mut self, mut amphipod: Amphipod, dst: &Coord) -> u32 {
        let (x, y) = dst;
        let energy = amphipod.move_to(dst);
        self.map[*y as usize][*x as usize] = Some(amphipod);
        energy
    }

    fn is_solved(&self) -> bool {
        for r in self.map.iter() {
            for a in r.iter() {
                match a {
                    Some(a) if !a.in_own_room() => return false,
                    _ => (),
                }
            }
        }
        true
    }

    fn key(&self) -> String {
        self.map
            .iter()
            .flat_map(|r| {
                r.iter().map(|oa| match oa {
                    None => ".".to_owned(),
                    Some(a) => a.name.to_string(),
                })
            })
            .collect()
    }

    fn solve(&mut self, energy: u32, mut minimum: u32, dejavu: &mut HashMap<String, u32>) -> u32 {
        if self.is_solved() {
            // println!("Found {} - minimum before {} with dejavu #{}", energy, minimum, dejavu.len());
            min(energy, minimum)
        } else {
            for (x, y) in self.positions().into_iter() {
                let mut a = self.move_out(&(x, y)); // out of old spot
                if a.needs_to_move(self) {
                    let moves = a.possible_moves(self);
                    for dst in moves.into_iter() {
                        let new_energy = energy + self.move_in(a, &dst); // into new spot
                        let key = self.key();
                        if !dejavu.contains_key(&key) || dejavu[&key] > new_energy {
                            dejavu.insert(key, new_energy);
                            let e = self.solve(new_energy, minimum, dejavu);
                            minimum = min(e, minimum);
                        }
                        a = self.move_out(&dst); // out of new spot
                        a.pos = (x, y); //restore old location
                    }
                }
                self.move_in(a, &(x, y)); // into old spot
            }
            minimum
        }
    }
}

fn base_amphipods(room_size: u8) -> Vec<Amphipod> {
    vec![
        Amphipod { name: 'A', pos: (2, 1) },
        Amphipod { name: 'D', pos: (4, 1) },
        Amphipod { name: 'B', pos: (6, 1) },
        Amphipod { name: 'D', pos: (8, 1) },
        Amphipod { name: 'B', pos: (2, room_size) },
        Amphipod { name: 'C', pos: (4, room_size) },
        Amphipod { name: 'A', pos: (6, room_size) },
        Amphipod { name: 'C', pos: (8, room_size) },
    ]
}

fn extra_amphipods() -> Vec<Amphipod> {
    vec![
        Amphipod { name: 'D', pos: (2, 2) },
        Amphipod { name: 'C', pos: (4, 2) },
        Amphipod { name: 'B', pos: (6, 2) },
        Amphipod { name: 'A', pos: (8, 2) },
        Amphipod { name: 'D', pos: (2, 3) },
        Amphipod { name: 'B', pos: (4, 3) },
        Amphipod { name: 'A', pos: (6, 3) },
        Amphipod { name: 'C', pos: (8, 3) },
    ]
}

fn part1() -> u32 {
    let amphipods = base_amphipods(2);
    let mut floor = Floor::new(amphipods, 2);
    floor.solve(0, u32::MAX, &mut HashMap::new())
}

fn part2() -> u32 {
    let mut amphipods = base_amphipods(4);
    amphipods.append(&mut extra_amphipods());
    let mut floor = Floor::new(amphipods, 4);
    floor.solve(0, u32::MAX, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use crate::day23::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 12240);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 44618);
    }
}
