#!/usr/bin/env python3
import itertools
from collections import Counter
from typing import Tuple, Dict


def play_d100(pos):
    pos = [pos[0] - 1, pos[1] - 1]
    scores = [0, 0]
    die = 0
    player = 0
    rolls = 0
    while True:
        rolls += 3
        pos[player] = (pos[player] + 3 * die + 6) % 10
        scores[player] += pos[player] + 1
        if scores[player] >= 1000:
            break
        die = (die + 3) % 100
        player = 1 - player
    return rolls * scores[1 - player]


# 2*position, 2*score
Universe = Tuple[int, int, int, int]


def play_d3(pos):
    universes: Dict[Universe, int] = {(pos[0] - 1, pos[1] - 1, 0, 0): 1}
    finished: Dict[Universe, int] = {}
    combos = itertools.product([1, 2, 3], repeat=3)
    moves = Counter([sum(c) for c in combos])
    player = 0
    while len(universes):
        new_universes = {}
        for u, uni_cnt in universes.items():
            for move, mov_cnt in moves.items():
                new_pos = (u[player] + move) % 10
                new_score = u[player + 2] + new_pos + 1
                if player == 0:
                    new_u = new_pos, u[1], new_score, u[3]
                else:
                    new_u = u[0], new_pos, u[2], new_score
                cnt = uni_cnt * mov_cnt
                if new_score >= 21:
                    finished[new_u] = finished.get(new_u, 0) + cnt
                else:
                    new_universes[new_u] = new_universes.get(new_u, 0) + cnt
        player = 1 - player
        universes = new_universes
    win = [0, 0]
    for (_, _, s1, s2), cnt in finished.items():
        win[0 if s1 > s2 else 1] += cnt
    return max(win)


if __name__ == "__main__":
    start = [10, 9]

    score = play_d100(start)
    print(f'Score = {score}')
    assert score == 918081

    score = play_d3(start)
    print(f'Score = {score}')
    assert score == 158631174219251
