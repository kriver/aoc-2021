#!/usr/bin/env python3
import itertools
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
                    if cuboid.is_on():
                        m.add(c)
                    elif c in m:
                        m.remove(c)
    return m


class Cuboid:
    def __init__(self, on_off, c1: Coord, c2: Coord):
        self._on_off = on_off
        self._c1 = c1
        self._c2 = c2

    def __repr__(self):
        (x1, y1, z1), (x2, y2, z2) = self._c1, self._c2
        return f' ({x1},{y1},{z1})---({x2},{y2},{z2}) [vol={self.volume()}] '

    def __hash__(self):
        return hash((self._on_off, self._c1, self._c2))

    def __eq__(self, other: 'Cuboid'):
        return (self.is_on(), self._c1, self._c2) \
               == (other.is_on(), other.c1(), other.c2())

    @staticmethod
    def parse(line: str) -> 'Cuboid':
        m = re.match(LINE_REGEX, line)
        if not m:
            raise Exception('invalid input: ' + line)
        return Cuboid(m.group(1),
                      (int(m.group(2)), int(m.group(4)), int(m.group(6))),
                      (int(m.group(3)), int(m.group(5)), int(m.group(7))))

    def is_on(self):
        return 'on' == self._on_off

    def c1(self):
        return self._c1

    def c2(self):
        return self._c2

    def volume(self) -> int:
        (x1, y1, z1), (x2, y2, z2) = self._c1, self._c2
        return abs(x2 - x1 + 1) * abs(y2 - y1 + 1) * abs(z2 - z1 + 1)

    def contained_in(self, other: 'Cuboid') -> bool:
        (x1a, y1a, z1a), (x2a, y2a, z2a) = self._c1, self._c2
        (x1b, y1b, z1b), (x2b, y2b, z2b) = other._c1, other._c2
        return (x1b <= x1a and x2a <= x2b) \
               and (y1b <= y1a and y2a <= y2b) \
               and (z1b <= z1a and z2a <= z2b)

    def disjoint(self, other: 'Cuboid') -> bool:
        (x1a, y1a, z1a), (x2a, y2a, z2a) = self._c1, self._c2
        (x1b, y1b, z1b), (x2b, y2b, z2b) = other._c1, other._c2
        return (x2a < x1b or x2b < x1a) \
               and (y2a < y1b or y2b < y1a) \
               and (z2a < z1b or z2b < z1a)

    def intersection(self, other: 'Cuboid') -> 'Cuboid':
        (x1a, y1a, z1a), (x2a, y2a, z2a) = self._c1, self._c2
        (x1b, y1b, z1b), (x2b, y2b, z2b) = other._c1, other._c2
        return Cuboid(self._on_off,
                      (max(x1a, x1b), max(y1a, y1b), max(z1a, z1b)),
                      (min(x2a, x2b), min(y2a, y2b), min(z2a, z2b)))


def filter_fully_contained(cuboids: Set[Cuboid], add_sub, num_on) -> Tuple[int, Set[Cuboid]]:
    filtered = set()
    for c1 in cuboids:
        contained = False
        for c2 in (cuboids - {c1}):
            if c1.contained_in(c2):
                contained = True
                break
        if not contained:
            filtered.add(c1)
        else:
            num_on -= add_sub * c1.volume()
    return num_on, filtered


def solve_combos_1(combinations: List[Tuple[Cuboid, Cuboid]], add_sub, num_on):
    next_cuboids: Set[Cuboid] = set()
    for c1, c2 in combinations:
        if not c1.disjoint(c2):
            overlap = c1.intersection(c2)
            if overlap not in next_cuboids:
                num_on += add_sub * overlap.volume()  # FIXME on vs off
                next_cuboids.add(overlap)
    return num_on, next_cuboids


def solve_combos(combinations: List[Tuple[Cuboid, Cuboid]], add_sub, num_on):
    next_cuboids = set()
    while combinations:
        add_sub *= -1
        num_on, next_cuboids = solve_combos_1(combinations, add_sub, num_on)
        # num_on, next_cuboids = filter_fully_contained(next_cuboids, add_sub, num_on)
        combinations = list(itertools.combinations(next_cuboids, 2))
    # print(f'---last = {next_cuboids.pop() if next_cuboids else -1}')
    return num_on


def solve(cuboids):
    done = []
    num_on = 0
    for i, c in enumerate(cuboids):
        print(f'Processing line {i + 1} - {c} - ON: {num_on}')
        add_sub = (+1 if c.is_on() else -1)
        num_on += add_sub * c.volume()
        combinations: List[Tuple[Cuboid, Cuboid]] = \
            list(itertools.product(done, [c]))
        num_on = solve_combos(combinations, add_sub, num_on)
        done.append(c)
    return num_on


def verify(data, start, n):
    for i in range(start, n + 1):
        print(f'Verify {i}')
        map = to_map(data[:i])
        num_on = solve(data[:i])
        print(f'old vs new : {len(map)} - {num_on}')
        assert len(map) == num_on


def main():
    data = load('day22.txt', mapper=Cuboid.parse)

    n = 20
    verify(data, 6, n)
    # print(f'Verify {n}')
    # map = to_map(data[:n])
    # print(f'Old move (small) = {len(map)}')
    # num_on = solve(data[:n])
    # print(f'New move (small) = {num_on}')

    # map = to_map(data[:20])
    # print(f'On cubes (small) = {len(map)}')
    # assert len(map) == 650099


if __name__ == "__main__":
    main()
