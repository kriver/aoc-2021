#!/usr/bin/env python3
import itertools
from collections import Counter
from functools import reduce
from typing import Tuple, List, Set, Dict

from util import load

Coord = Tuple[int, int, int]


class Scanner:
    def __init__(self, scanner_id: int, scans: List[Coord]):
        self._id: int = scanner_id
        self._beacons: List[Coord] = scans
        self._counts: Dict[Coord, int] = {c: 1 for c in self._beacons}
        self._dist_list: List[int] = [self._dist(p1, p2) for p1, p2 in itertools.combinations(scans, 2)]
        self._dist_set: Set[int] = set(self._dist_list)
        # assert len(self._dist_list) == len(self._dist_set)
        if len(self._dist_list) != len(self._dist_set):
            print(f'Problem with scanner {self._id}')

    def __repr__(self):
        return f'Scanner[{self._id}/{len(self._beacons)}]'

    @staticmethod
    def _dist(p1: Coord, p2: Coord) -> int:
        x1, y1, z1 = p1
        x2, y2, z2 = p2
        return (x1 - x2) ** 2 + (y1 - y2) ** 2 + (z1 - z2) ** 2

    def _to_beacons(self, dist):
        idx = self._dist_list.index(dist)
        n = len(self._beacons) - 1
        first = 0
        while idx >= n:
            first += 1
            idx -= n
            n -= 1
        p1 = self._beacons[first]
        p2 = self._beacons[first + idx + 1]
        # assert self._dist(p1, p2) == dist
        return {p1, p2}

    def _increment_all(self, overlap: Set[int]):
        beacons = reduce(lambda acc, dist: acc | self._to_beacons(dist), overlap, set())
        for b in beacons:
            self._counts[b] += 1

    def num_beacons(self):
        return len(self._beacons)

    def beacon_counts(self):
        return self._counts.values()

    def calc_overlaps(self, other: 'Scanner'):
        print(f'Checking {self} and {other}')
        overlap = self._dist_set & other._dist_set
        if len(overlap) >= 66:  # 12 points -> 66 distances (11+10+9+...+1)
            print(f'Overlap of {len(overlap)} distances for {self} and {other}')
            self._increment_all(overlap)
            other._increment_all(overlap)


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

    total_scanned_beacons = sum([s.num_beacons() for s in scanners])
    for scanner1, scanner2 in itertools.combinations(scanners, 2):
        scanner1.calc_overlaps(scanner2)

    all_counts = reduce(lambda acc, l: acc + l, [list(s.beacon_counts()) for s in scanners])
    grouped_counts = Counter(all_counts)
    assert sum(grouped_counts.values()) == total_scanned_beacons
    unique_counts = [v / k for k, v in grouped_counts.items()]
    unique = sum(unique_counts)

    print(f'Total beacons = {total_scanned_beacons}')
    print(f'Unique beacons = {unique}')
    assert unique == 403
