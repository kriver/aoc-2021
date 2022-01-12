use std::collections::HashMap;

use crate::util::load;

type Pixel = u8;
type OnOff = Vec<Pixel>;
type CType = i32;
type Coord = (CType, CType);
type Grid = HashMap<Coord, Pixel>;

static DIM: CType = 100;

fn input() -> (OnOff, Grid) {
    let lines: Vec<String> = load("data/day20.txt");
    let on_off = lines[0].chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();
    let grid = lines.into_iter()
        .skip(2)
        .enumerate()
        .fold(HashMap::new(),
              |acc, (y, line)| line.chars()
                  .enumerate()
                  .fold(acc,
                        |mut acc, (x, c)| {
                            acc.insert((x as CType, y as CType),
                                       if '#' == c { 1 } else { 0 });
                            acc
                        },
                  ),
        );
    (on_off, grid)
}

fn new_value(on_off: &OnOff, grid: &Grid, (x, y): Coord, default: usize) -> Pixel {
    let idx = (-1..=1)
        .fold(0,
              |acc, dy| (-1..=1)
                  .fold(acc,
                        |acc, dx|
                            acc * 2 + (
                                match grid.get(&(x + dx, y + dy)) {
                                    Some(v) => *v as usize,
                                    None => on_off[default] as usize,
                                }),
                  ));
    on_off[idx]
}

fn evolve_1(on_off: &OnOff, grid: Grid, delta: CType, default: usize) -> Grid {
    let mut new_grid = HashMap::new();
    for y in -delta..DIM + delta {
        for x in -delta..DIM + delta {
            new_grid.insert((x, y), new_value(on_off, &grid, (x, y), default));
        }
    }
    new_grid
}

fn evolve(on_off: &OnOff, mut grid: Grid, generations: usize) -> Grid {
    let mut delta = 1;
    let mut default = if on_off[0] == 0 { 0 } else { 511 };
    for _ in 0..generations {
        grid = evolve_1(on_off, grid, delta, default);
        delta += 1;
        default = if on_off[default] == 1 { 511 } else { 0 };
    }
    grid
}

fn part1(on_off: OnOff, mut grid: Grid) -> usize {
    grid = evolve(&on_off, grid, 2);
    grid.into_values().filter(|v| *v == 1).count()
}

fn part2(on_off: OnOff, mut grid: Grid) -> usize {
    grid = evolve(&on_off, grid, 50);
    grid.into_values().filter(|v| *v == 1).count()
}

#[cfg(test)]
mod tests {
    use crate::day20::{input, part1, part2};

    #[test]
    fn test_part1() {
        let (on_off, grid) = input();
        assert_eq!(part1(on_off, grid), 5464);
    }

    #[test]
    fn test_part2() {
        let (on_off, grid) = input();
        assert_eq!(part2(on_off, grid), 19228);
    }
}
