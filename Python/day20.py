#!/usr/bin/env python3
from typing import Tuple, Dict

from util import load

Coord = Tuple[int, int]
Algorithm = Dict[int, bool]
Grid = Dict[Coord, int]

DIM = 0


def new_value(g: Grid, algo: Algorithm, c: Coord, default: int) -> int:
    x, y = c
    idx = 0
    for dy in [-1, 0, 1]:
        for dx in [-1, 0, 1]:
            nc = (x + dx, y + dy)
            idx = idx * 2 + g.get(nc, algo[default])
    return algo[idx]


def evolve_1(g: Grid, algo: Algorithm, delta, default) -> Grid:
    ng = {}
    search_delta = delta + 1
    for y in range(-search_delta, DIM + search_delta):
        for x in range(-search_delta, DIM + search_delta):
            lit = new_value(g, algo, (x, y), default)
            ng[(x, y)] = lit
    return ng


def evolve(g: Grid, algo: Algorithm, steps: int) -> Grid:
    delta = 0
    default = 0 if algo[0] == 0 else 511
    for i in range(steps):
        print(f'Lit pixels after {i} enhancements = {lit_pixels(g)}, '
              f'with delta {delta} and default {default}')
        # display(g, delta)
        g = evolve_1(g, algo, delta, default)
        delta += 1
        default = 511 if algo[default] else 0
    return g


def lit_pixels(g: Grid) -> int:
    return sum(filter(lambda p: p == 1, g.values()))


def display(g: Grid, delta: int):
    ls = [
        [('#' if g.get((x, y), 0) else '.')
         for x in range(-delta, DIM + delta)]
        for y in range(-delta, DIM + delta)]
    print(f'Delta = {delta}')
    for l in ls:
        print(''.join(l))


if __name__ == "__main__":
    lines = load('day20.txt')
    algorithm: Algorithm = {
        i: (1 if c == '#' else 0) for i, c in enumerate(lines[0])}
    grid: Grid = {
        (x, y): (1 if c == '#' else 0)
        for y, line in enumerate(lines[2:])
        for x, c in enumerate(line)
    }
    DIM = len(lines[2])

    result = evolve(grid, algorithm, 2)
    num_lit = lit_pixels(result)
    print(f'Lit pixels after 2 enhancements = {num_lit}')
    assert num_lit == 5464

    result = evolve(grid, algorithm, 50)
    num_lit = lit_pixels(result)
    print(f'Lit pixels after 50 enhancements = {num_lit}')
    assert num_lit == 19228
