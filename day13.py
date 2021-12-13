#!/usr/bin/env python3
from typing import Tuple, Set

from util import *

Coord = Tuple[int, int]
Dots = Set[Coord]


def parse_dots(lines) -> Tuple[int, Dots]:
    coords = set()
    idx = 0
    for line in lines:
        if len(line) == 0:
            break
        idx += 1
        c = [int(i) for i in line.split(',')]
        coords.add((c[0], c[1]))
    return idx + 1, coords


def fold_x(dots: Dots, fold_at: int) -> Dots:
    return {(2 * fold_at - x, y) if x > fold_at else (x, y) for x, y in dots}


def fold_y(dots: Dots, fold_at: int) -> Dots:
    return {(x, 2 * fold_at - y) if y > fold_at else (x, y) for x, y in dots}


def fold(dots: Dots, instr: str) -> Dots:
    s = instr.split('=')
    if s[0] == 'fold along x':
        return fold_x(dots, int(s[1]))
    else:
        return fold_y(dots, int(s[1]))


def fold_all(dots: Dots, instrs: List[str]) -> Dots:
    for instr in instrs:
        dots = fold(dots, instr)
    return dots


def display(dots: Dots):
    lines = [['.' for x in range(40)] for y in range(6)]
    for x, y in dots:
        lines[y][x] = '#'
    for line in lines:
        print(''.join(line))


if __name__ == "__main__":
    data = load('day13.txt')
    start_instructions, all_dots = parse_dots(data)

    result = fold(all_dots, data[start_instructions])
    print("Number of dots: %d" % len(result))
    assert len(result) == 669

    result = fold_all(all_dots, data[start_instructions:])
    print("Number of dots: %d" % len(result))
    assert len(result) == 90

    display(result)
