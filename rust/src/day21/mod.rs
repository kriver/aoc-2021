use std::cmp::max;
use std::collections::HashMap;

use itertools::{iproduct, Itertools};

fn part1(p1: u32, p2: u32) -> u32 {
    let mut pos = [p1 - 1, p2 - 1];
    let mut score = [0, 0];
    let mut player = 0;
    let mut die = 0;
    let mut rolls = 0;
    loop {
        rolls += 3;
        pos[player] = (pos[player] + 3 * die + 6) % 10;
        score[player] += pos[player] + 1;
        if score[player] >= 1000 {
            break;
        }
        die = (die + 3) % 100;
        player = 1 - player;
    }
    rolls * score[1 - player]
}

fn part2(p1: u32, p2: u32) -> u64 {
    type Universe = [u32; 4]; // pos1, pos2, score1, score2
    type UMap = HashMap<Universe, u64>;

    let mut universes: UMap = HashMap::from([([p1 - 1, p2 - 1, 0, 0], 1)]);
    let mut finished: UMap = HashMap::new();
    let nums = [1, 2, 3];
    let moves = iproduct!(nums.iter(), nums.iter(), nums.iter())
        .map(|n| n.0 + n.1 + n.2)
        .counts();
    let mut player = 0;
    while !universes.is_empty() {
        let mut new_universes = HashMap::new();
        for (u, uni_cnt) in universes.into_iter() {
            for (m, mov_cnt) in moves.iter() {
                let new_pos = (u[player] + m) % 10;
                let new_score = u[player + 2] + new_pos + 1;
                let new_u = match player {
                    0 => [new_pos, u[1], new_score, u[3]],
                    1 => [u[0], new_pos, u[2], new_score],
                    _ => unreachable!(),
                };
                let cnt = uni_cnt * (*mov_cnt as u64);
                if new_score >= 21 {
                    *finished.entry(new_u).or_default() += cnt;
                } else {
                    *new_universes.entry(new_u).or_default() += cnt;
                }
            }
        }
        player = 1 - player;
        universes = new_universes
    }
    let mut win = [0, 0];
    for (u, cnt) in finished.into_iter() {
        if u[2] > u[3] { win[0] += cnt; } else { win[1] += cnt; }
    }
    max(win[0], win[1])
}

#[cfg(test)]
mod tests {
    use crate::day21::{part1, part2};

    const PLAYER_1: u32 = 10;
    const PLAYER_2: u32 = 9;

    #[test]
    fn test_part1() {
        assert_eq!(part1(PLAYER_1, PLAYER_2), 918081);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PLAYER_1, PLAYER_2), 158631174219251);
    }
}
