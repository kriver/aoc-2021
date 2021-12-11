#!/usr/bin/env python3
from typing import Tuple

import numpy as np

from util import *

Coord = Tuple[int, int]
DX = DY = 10


def increment_energy_levels(levels):
    new_levels = levels + np.ones((DX, DY))
    will_flash = np.where(new_levels == 10)
    return new_levels, list(zip(will_flash[0], will_flash[1]))


def reset_flashed(levels, coords):
    for y, x in coords:
        levels[y, x] = 0


def flash_inc(levels, coord: Coord) -> bool:
    y, x = coord
    if levels[y, x] < 10:
        levels[y, x] += 1
        return levels[y, x] == 10
    else:
        return False


def flash(levels, coord: Coord, will_flash, has_flashed):
    new_will_flash = []
    y, x = coord
    for dy in [-1, 0, 1]:
        for dx in [-1, 0, 1]:
            if dx == dy == 0:
                continue
            new_coord = y + dy, x + dx
            if new_coord[0] < 0 or new_coord[0] >= DY \
                    or new_coord[1] < 0 or new_coord[1] >= DX:
                continue
            if new_coord in will_flash or new_coord in has_flashed:
                continue
            if flash_inc(levels, new_coord):
                new_will_flash.append(new_coord)
    return new_will_flash


def cycle_1(levels):
    has_flashed = []
    levels, will_flash = increment_energy_levels(levels)
    while will_flash:
        coord = will_flash.pop()
        has_flashed.append(coord)
        will_flash.extend(flash(levels, coord, will_flash, has_flashed))
    reset_flashed(levels, has_flashed)
    return levels, has_flashed


def cycle(levels, cycles: int) -> int:
    flash_cnt = 0
    for c in range(cycles):
        levels, has_flashed = cycle_1(levels)
        flash_cnt += len(has_flashed)
    return flash_cnt


def all_flash(levels) -> int:
    step = 0
    while True:
        step += 1
        levels, has_flashed = cycle_1(levels)
        if len(has_flashed) == DX * DY:
            break
    return step


if __name__ == "__main__":
    data = load('day11.txt')
    energy_levels = np.array([[int(h) for h in line] for line in data])

    flashes = cycle(energy_levels, 100)
    print('Number of flashes: %d' % flashes)
    assert flashes == 1652

    steps = all_flash(energy_levels)
    print('Step when all flash: %d' % steps)
    assert steps == 220
