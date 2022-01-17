use std::collections::HashSet;

use crate::util::load;

type Coord = (usize, usize);
type Cucumbers = HashSet<Coord>;

const DIM_X: usize = 139;
const DIM_Y: usize = 137;

fn input() -> (Cucumbers, Cucumbers) {
    let lines: Vec<String> = load("data/day25.txt");
    let mut east: Cucumbers = HashSet::new();
    let mut south: Cucumbers = HashSet::new();
    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '>' => east.insert((x, y)),
                'v' => south.insert((x, y)),
                _ => true,
            };
        }
    }
    (east, south)
}

fn move_once(east: Cucumbers, south: Cucumbers) -> (u32, Cucumbers, Cucumbers) {
    let mut moved = 0;
    let mut new_east: Cucumbers = HashSet::new();
    let mut new_south: Cucumbers = HashSet::new();
    for (mut x, y) in east.iter() {
        let nx = (x + 1) % DIM_X;
        if !east.contains(&(nx, *y)) && !south.contains(&(nx, *y)) {
            moved += 1;
            x = nx;
        }
        new_east.insert((x, *y));
    }
    for (x, mut y) in south.iter() {
        let ny = (y + 1) % DIM_Y;
        if !new_east.contains(&(*x, ny)) && !south.contains(&(*x, ny)) {
            moved += 1;
            y = ny;
        }
        new_south.insert((*x, y));
    }
    (moved, new_east, new_south)
}

fn part1(mut east: Cucumbers, mut south: Cucumbers) -> u32 {
    let mut cnt = 0;
    loop {
        cnt += 1;
        let (moved, new_east, new_south) = move_once(east, south);
        if moved == 0 {
            break cnt;
        }
        if cnt % 100 == 0 {
            println!("Step {}, {} moved", cnt, moved);
        }
        east = new_east;
        south = new_south;
    }
}

#[cfg(test)]
mod tests {
    use crate::day25::{input, part1};

    #[test]
    fn test_part1() {
        let (east, south) = input();
        assert_eq!(part1(east, south), 486);
    }
}
