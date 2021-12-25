#!/usr/bin/env python3

from util import load

DIM_X = 139
DIM_Y = 137


def move(east, south):
    moved = 0
    new_east = set()
    for pos in east:
        x, y = pos
        npos = (x + 1) % DIM_X, y
        if npos not in east and npos not in south:
            moved += 1
            new_east.add(npos)
        else:
            new_east.add(pos)
    new_south = set()
    for pos in south:
        x, y = pos
        npos = x, (y + 1) % DIM_Y
        if npos not in new_east and npos not in south:
            moved += 1
            new_south.add(npos)
        else:
            new_south.add(pos)
    return new_east, new_south, moved


def solve1():
    east = {
        (x, y)
        for y, line in enumerate(load('day25.txt'))
        for x, c in enumerate(list(line))
        if c == '>'}
    south = {
        (x, y)
        for y, line in enumerate(load('day25.txt'))
        for x, c in enumerate(list(line))
        if c == 'v'}

    step = 0
    while True:
        step += 1
        east, south, moved = move(east, south)
        if not moved:
            break
        if not step % 100:
            print(f'Step {step}, {moved} moved')

    print(f'Steps = {step}')
    assert step == 486


if __name__ == "__main__":
    solve1()
