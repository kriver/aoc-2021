#!/usr/bin/env python3
from typing import Tuple, Dict

from sortedcontainers import SortedList

from util import *

Coord = Tuple[int, int]
Map = Dict[Coord, Tuple[int, int]]  # position -> risk, total
Segment = [int, Coord]  # total risk, current node

DELTA = [(-1, 0), (1, 0), (0, -1), (0, 1)]


def sorting_key(s: Segment) -> int:
    return s[0]


def find_path(risks: Map, start: Coord, end: Coord) -> int:
    visited = {start}
    queue: SortedList[Segment] = SortedList([(0, start)], key=sorting_key)
    while True:
        total_risk, (x, y) = queue.pop(0)
        if (x, y) == end:
            return total_risk
        for dx, dy in DELTA:
            new_pos = x + dx, y + dy
            if new_pos in risks and new_pos not in visited:
                risk, prev_total = risks[new_pos]
                new_risk = total_risk + risk
                if prev_total == -1 or new_risk < prev_total:
                    risks[new_pos] = risk, new_risk
                queue.add((new_risk, new_pos))
                visited.add(new_pos)


if __name__ == "__main__":
    data = load('day15.txt')
    risk_map = {(x, y): (int(h), -1)
                for y, line in enumerate(data)
                for x, h in enumerate(line)}

    best_path = find_path(risk_map, (0, 0), (99, 99))
    print('Lowest risk: %d' % best_path)
    assert best_path == 373

    risk_map = {
        (mx * len(line) + x, my * len(data) + y):
            ((int(h) - 1 + mx + my) % 9 + 1, -1)
        for my in range(5)
        for y, line in enumerate(data)
        for mx in range(5)
        for x, h in enumerate(line)}

    best_path = find_path(risk_map, (0, 0), (499, 499))
    print('Lowest risk: %d' % best_path)
    assert best_path == 2868
