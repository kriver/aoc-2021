use std::cmp::{max, min};

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

const BOTTOM_LEFT: Coord = Coord { x: 60, y: -171 };
const TOP_RIGHT: Coord = Coord { x: 94, y: -136 };

fn step(pos: &Coord, velocity: &Coord) -> (Coord, Coord) {
    (Coord { x: pos.x + velocity.x, y: pos.y + velocity.y },
     Coord { x: max(velocity.x - 1, 0), y: velocity.y - 1 })
}

fn in_area(pos: &Coord) -> bool {
    BOTTOM_LEFT.x <= pos.x && pos.x <= TOP_RIGHT.x &&
        BOTTOM_LEFT.y <= pos.y && pos.y <= TOP_RIGHT.y
}

fn overshot(pos: &Coord) -> bool {
    pos.x > max(BOTTOM_LEFT.x, TOP_RIGHT.x) ||
        pos.y < min(BOTTOM_LEFT.y, TOP_RIGHT.y)
}

fn move_until_done(velocity: &Coord) -> Option<i32> {
    let mut pos = Coord { x: 0, y: 0 };
    let mut v = *velocity;
    let mut highest = 0;
    loop {
        if in_area(&pos) {
            break Some(highest);
        } else if overshot(&pos) {
            break None;
        }
        (pos, v) = step(&pos, &v);
        if pos.y > highest {
            highest = pos.y
        }
    }
}

// Solve quadratic equation satisfying:
//   vx+(vx-1)+...+2+1+0 = (vx+1)*(vx/2) >= x
fn vx_for(x: i32) -> i32 {
    ((-1.0 + (1.0 + 8.0 * x as f32).sqrt()) / 2.0).ceil() as i32
}

fn find_highest() -> Option<i32> {
    let mut velocity = Coord { x: vx_for(BOTTOM_LEFT.x), y: -(TOP_RIGHT.y + 1) };
    let mut highest = None;
    loop {
        highest = match move_until_done(&velocity) {
            Some(new) => match highest {
                None => Some(new),
                Some(old)if new >= old => Some(new),
                Some(old) => Some(old),
            }
            None => highest,
        };
        if velocity.y > BOTTOM_LEFT.y.abs() + 1 {
            break highest;
        }
        velocity.y += 1;
    }
}

fn find_all() -> usize {
    let mut cnt = 0;
    for y in BOTTOM_LEFT.y - 1..-BOTTOM_LEFT.y + 1 {
        for x in vx_for(BOTTOM_LEFT.x)..TOP_RIGHT.x + 1 {
            let velocity = Coord { x, y };
            if let Some(_) = move_until_done(&velocity) {
                cnt += 1
            }
        }
    }
    cnt
}

fn part1() -> Option<i32> {
    find_highest()
}

fn part2() -> usize {
    find_all()
}

#[cfg(test)]
mod tests {
    use crate::day17::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), Some(14535));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2270);
    }
}
