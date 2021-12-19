#!/usr/bin/env python3
import itertools
from collections import Counter
from typing import Tuple, List, Set

import numpy as np

from util import load

Coord = Tuple[int, int, int]

ROTATIONS = [np.array([[-1, 0, 0], [0, -1, 0], [0, 0, 1]]),
             np.array([[-1, 0, 0], [0, 1, 0], [0, 0, -1]]),
             np.array([[1, 0, 0], [0, -1, 0], [0, 0, -1]]),
             np.array([[1, 0, 0], [0, 1, 0], [0, 0, 1]]),
             np.array([[-1, 0, 0], [0, 0, -1], [-0, -1, -0]]),
             np.array([[-1, 0, 0], [0, 0, 1], [0, 1, -0]]),
             np.array([[1, 0, 0], [0, 0, -1], [-0, 1, 0]]),
             np.array([[1, 0, 0], [0, 0, 1], [0, -1, 0]]),
             np.array([[0, -1, 0], [-1, 0, 0], [-0, -0, -1]]),
             np.array([[0, -1, 0], [1, 0, 0], [-0, 0, 1]]),
             np.array([[0, 1, 0], [-1, 0, 0], [0, -0, 1]]),
             np.array([[0, 1, 0], [1, 0, 0], [0, 0, -1]]),
             np.array([[0, -1, 0], [0, 0, -1], [1, 0, 0]]),
             np.array([[0, -1, 0], [0, 0, 1], [-1, 0, 0]]),
             np.array([[0, 1, 0], [0, 0, -1], [-1, 0, 0]]),
             np.array([[0, 1, 0], [0, 0, 1], [1, 0, 0]]),
             np.array([[0, 0, -1], [-1, 0, 0], [0, 1, 0]]),
             np.array([[0, 0, -1], [1, 0, 0], [0, -1, 0]]),
             np.array([[0, 0, 1], [-1, 0, 0], [0, -1, 0]]),
             np.array([[0, 0, 1], [1, 0, 0], [0, 1, 0]]),
             np.array([[0, 0, -1], [0, -1, 0], [-1, -0, -0]]),
             np.array([[0, 0, -1], [0, 1, 0], [1, -0, 0]]),
             np.array([[0, 0, 1], [0, -1, 0], [1, 0, -0]]),
             np.array([[0, 0, 1], [0, 1, 0], [-1, 0, 0]])]


class Scanner:
    def __init__(self, scanner_id: int, scans: List[Coord]):
        self._id: int = scanner_id
        self._beacons: Set[Coord] = set(scans)
        self._dist_set: Set[int] = self._distances()

    def __repr__(self):
        return f'Scanner[{self._id}/{len(self._beacons)}]'

    @staticmethod
    def _dist(p1: Coord, p2: Coord) -> int:
        x1, y1, z1 = p1
        x2, y2, z2 = p2
        return (x1 - x2) ** 2 + (y1 - y2) ** 2 + (z1 - z2) ** 2

    @staticmethod
    def _dist_tuple(p1: Coord, p2: Coord) -> Coord:
        x1, y1, z1 = p1
        x2, y2, z2 = p2
        return (x1 - x2), (y1 - y2), (z1 - z2)

    def num_beacons(self):
        return len(self._beacons)

    def overlaps(self, other: 'Scanner'):
        return len(self._dist_set & other._dist_set) >= 12

    def combine(self, other: 'Scanner'):
        rotations = other._all_rotations()
        for rotation in rotations:
            distances = [self._dist_tuple(beacon, other_beacon)
                         for other_beacon in rotation
                         for beacon in self._beacons]
            counted = Counter(distances)
            for distance, count in counted.items():
                if count >= 12:
                    self._add(rotation, distance)
                    return True
        return False

    def _distances(self):
        return set([self._dist(p1, p2) for p1, p2 in itertools.combinations(self._beacons, 2)])

    def _all_rotations(self) -> List[List[Coord]]:
        return [[tuple(np.dot(rotation, np.array(beacon)))
                 for beacon in self._beacons]
                for rotation in ROTATIONS]

    def _add(self, other: List[Coord], dist: Coord):
        dx, dy, dz = dist
        moved = {(x + dx, y + dy, z + dz) for x, y, z in other}
        self._beacons |= moved
        # FIXME optimize
        self._dist_set = self._distances()


def parse(lines: List[str]) -> List[Scanner]:
    result = []
    scanner_scans = []
    scanner_id = -1
    for line in lines:
        if len(line) == 0:
            result.append(Scanner(scanner_id, scanner_scans))
            continue
        elif line.startswith('--- scanner'):
            scanner_scans = []
            scanner_id = int(line.split()[2])
        else:
            scanner_scans.append(tuple([int(n) for n in line.split(',')]))
    result.append(Scanner(scanner_id, scanner_scans))
    return result


if __name__ == "__main__":
    scanners = parse(load('day19.txt'))

    solution = scanners[0]
    solved = [scanners[0]]
    unsolved = scanners[1:]
    while unsolved:
        before = len(unsolved)
        for u in unsolved:
            for s in solved:
                if s.overlaps(u):
                    if solution.combine(u):
                        solved.append(u)
                        unsolved.remove(u)
                        print(f'{solution}')
                        break
        assert before != len(unsolved)
    print(f'{solution}')

    unique = solution.num_beacons()
    print(f'Unique beacons = {unique}')
    assert unique == 403
