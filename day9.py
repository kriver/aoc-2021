#!/usr/bin/env python3
from typing import Tuple, Set

import numpy as np

from util import *

Coord = Tuple[int, int]


def find_low_points(hm) -> List[Coord]:
    dx, dy = hm.shape
    lp = []
    for y in range(dy):
        for x in range(dx):
            h = hm[y, x]
            if (x == 0 or hm[y, x - 1] > h) \
                    and (x == dx - 1 or hm[y, x + 1] > h) \
                    and (y == 0 or hm[y - 1, x] > h) \
                    and (y == dy - 1 or hm[y + 1, x] > h):
                lp.append((x, y))
    return lp


def calc_risk_level(lp: List[Coord], hm) -> int:
    rl = 0
    for x, y in lp:
        rl += hm[y, x] + 1
    return rl


def update_basin_r(coord: Coord, hm, basin: Set[Coord]):
    dx, dy = hm.shape
    if coord not in basin:
        x, y = coord
        if hm[y, x] < 9:
            basin.add(coord)
            if x > 0:
                update_basin_r((x - 1, y), hm, basin)
            if x < dx - 1:
                update_basin_r((x + 1, y), hm, basin)
            if y > 0:
                update_basin_r((x, y - 1), hm, basin)
            if y < dy - 1:
                update_basin_r((x, y + 1), hm, basin)


def calc_basin_size(coord: Coord, hm) -> int:
    basin = set()
    update_basin_r(coord, hm, basin)
    return len(basin)


def calc_basin_sizes(lp: List[Coord], hm) -> List[int]:
    return [calc_basin_size(c, hm) for c in lp]


if __name__ == "__main__":
    data = load('day9.txt')
    height_map = np.array([[int(h) for h in line] for line in data])

    low_points = find_low_points(height_map)
    print('Number of low points: %d' % len(low_points))
    risk_level = calc_risk_level(low_points, height_map)
    print('Total risk level: %d' % risk_level)
    assert risk_level == 480

    basin_sizes = calc_basin_sizes(low_points, height_map)
    sorted_sizes = list(reversed(sorted(basin_sizes)))
    result = sorted_sizes[0] * sorted_sizes[1] * sorted_sizes[2]
    print('Product of 3 largest basins sizes: %d' % result)
    assert result == 1045660
