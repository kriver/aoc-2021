#!/usr/bin/env python3
import re
from typing import Tuple, Set, List

from util import load

LINE_REGEX = re.compile(r'^(on|off) '
                        r'x=(.*)\.\.(.*),y=(.*)\.\.(.*),z=(.*)\.\.(.*)$')

Coord = Tuple[int, int, int]


def to_map(cuboids) -> Set[Coord]:
    m = set()
    for cuboid in cuboids:
        x1, y1, z1 = cuboid.c1()
        x2, y2, z2 = cuboid.c2()
        if x1 < -50 or x2 > 50 or y1 < -50 or y2 > 50 or z1 < -50 or z2 > 50:
            continue
        for z in range(z1, z2 + 1):
            for y in range(y1, y2 + 1):
                for x in range(x1, x2 + 1):
                    c = (x, y, z)
                    if cuboid.add_sub() == 1:
                        m.add(c)
                    elif c in m:
                        m.remove(c)
    return m


class Cuboid:
    def __init__(self, add_sub, c1: Coord, c2: Coord):
        self._c1 = c1
        self._c2 = c2
        self._add_sub = add_sub

    def __repr__(self):
        (x1, y1, z1), (x2, y2, z2) = self._c1, self._c2
        return f' [{self._add_sub}] ({x1},{y1},{z1})---({x2},{y2},{z2}) [vol={self.volume()}] '

    @staticmethod
    def parse(line: str) -> 'Cuboid':
        m = re.match(LINE_REGEX, line)
        if not m:
            raise Exception('invalid input: ' + line)
        return Cuboid(1 if m.group(1) == 'on' else -1,
                      (int(m.group(2)), int(m.group(4)), int(m.group(6))),
                      (int(m.group(3)), int(m.group(5)), int(m.group(7))))

    def c1(self):
        return self._c1

    def c2(self):
        return self._c2

    def add_sub(self):
        return self._add_sub

    def add_sub_invert(self):
        self._add_sub *= -1

    def volume(self) -> int:
        (x1, y1, z1), (x2, y2, z2) = self._c1, self._c2
        return abs(x2 - x1 + 1) * abs(y2 - y1 + 1) * abs(z2 - z1 + 1)

    def disjoint(self, other: 'Cuboid') -> bool:
        (x1a, y1a, z1a), (x2a, y2a, z2a) = self._c1, self._c2
        (x1b, y1b, z1b), (x2b, y2b, z2b) = other._c1, other._c2
        return (x2a < x1b or x2b < x1a) \
               or (y2a < y1b or y2b < y1a) \
               or (z2a < z1b or z2b < z1a)

    def intersection(self, other: 'Cuboid') -> 'Cuboid':
        (x1a, y1a, z1a), (x2a, y2a, z2a) = self._c1, self._c2
        (x1b, y1b, z1b), (x2b, y2b, z2b) = other._c1, other._c2
        return Cuboid(self.add_sub() * other.add_sub(),
                      (max(x1a, x1b), max(y1a, y1b), max(z1a, z1b)),
                      (min(x2a, x2b), min(y2a, y2b), min(z2a, z2b)))


def get_overlaps(done: List[Cuboid], other: Cuboid) -> List[Cuboid]:
    overlaps: List[Cuboid] = []
    for c in done:
        if not c.disjoint(other):
            overlap = c.intersection(other)
            if other.add_sub() == 1:  # switch if new one is 'on'
                overlap.add_sub_invert()
            overlaps.append(overlap)
    return overlaps


def solve(cuboids: List[Cuboid]):
    done = []
    for i, c in enumerate(cuboids):
        # print(f'Processing line {i + 1}')
        overlaps = get_overlaps(done, c)
        done.extend(overlaps)
        if c.add_sub() == 1:
            done.append(c)
    return sum([c.add_sub() * c.volume() for c in done])


def main():
    data: List[Cuboid] = load('day22.txt', mapper=Cuboid.parse)

    map = to_map(data[:20])
    print(f'On cubes (part 1) = {len(map)}')
    assert len(map) == 650099

    num = solve(data)
    print(f'On cubes (part 2) = {num}')
    assert num == 1254011191104293


if __name__ == "__main__":
    main()
