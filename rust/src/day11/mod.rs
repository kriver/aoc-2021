use crate::util::load;

type Octopi = Vec<Vec<u8>>;
type Coord = (u8, u8);
type FlashSet = Vec<Coord>;

fn input() -> Octopi {
    let lines: Vec<String> = load("data/day11.txt");
    lines.into_iter()
        .map(|l| l.chars().into_iter()
            .map(|c| c as u8 - '0' as u8)
            .collect())
        .collect()
}

fn incr_energy(octopi: &mut Octopi) -> FlashSet {
    let mut will_flash: FlashSet = Vec::new();
    for (y, l) in octopi.iter_mut().enumerate() {
        for (x, p) in l.iter_mut().enumerate() {
            *p += 1;
            if *p > 9 {
                will_flash.push((x as u8, y as u8));
            }
        }
    }
    will_flash
}

fn flash(octopi: &mut Octopi, (x, y): &Coord, will_flash: &mut FlashSet) {
    for dy in -1i8..=1 {
        for dx in -1i8..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let (nx, ny) = (*x as i8 + dx, *y as i8 + dy);
            if ny >= 0 && ny < octopi.len() as i8 && nx >= 0 && nx < octopi[0].len() as i8 {
                let (nx, ny) = (nx as usize, ny as usize);
                if octopi[ny][nx] <= 9 {
                    octopi[ny][nx] += 1;
                    if octopi[ny][nx] > 9 {
                        will_flash.push((nx as u8, ny as u8));
                    }
                }
            }
        }
    }
}

fn reset_flashed(octopi: &mut Octopi) {
    for l in octopi.iter_mut() {
        for p in l.iter_mut() {
            if *p > 9 {
                *p = 0;
            }
        }
    }
}

fn cycle_once(octopi: &mut Octopi) -> usize {
    let mut has_flashed: usize = 0;
    let mut will_flash = incr_energy(octopi);
    while !will_flash.is_empty() {
        let next = will_flash.swap_remove(0);
        flash(octopi, &next, &mut will_flash);
        has_flashed += 1;
    }
    reset_flashed(octopi);
    has_flashed
}

fn part1(octopi: &mut Octopi, cycles: u32) -> usize {
    (0..cycles)
        .map(|_i| cycle_once(octopi))
        .sum()
}

fn part2(octopi: &mut Octopi) -> u32 {
    let mut cycles = 0;
    loop {
        cycles += 1;
        if cycle_once(octopi) == 100 {
            break;
        }
    }
    cycles
}

#[cfg(test)]
mod tests {
    use crate::day11::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(&mut input(), 100), 1652);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&mut input()), 220);
    }
}
