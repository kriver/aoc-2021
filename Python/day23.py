#!/usr/bin/env python3
import sys
from typing import List

AMPHIPOD_ROOMS = {'A': 2, 'B': 4, 'C': 6, 'D': 8}
ENERGY = {'A': 1, 'B': 10, 'C': 100, 'D': 1000}
HALL_POSITIONS = {0, 1, 3, 5, 7, 9, 10}


def manhattan_distance(p1, p2):
    x1, y1 = p1
    x2, y2 = p2
    return abs(x1 - x2) + abs(y1 - y2)


def needed_energy(p1, p2, amph: str):
    return manhattan_distance(p1, p2) * ENERGY[amph[0]]


def amphipod(floor, pos):
    x, y = pos
    return floor[y][x]


def needs_to_move(floor, amph: str, pos):
    x, y = pos
    if AMPHIPOD_ROOMS[amph[0]] == x:
        # either we're at back of the room, or we're at the front and
        # our friend is already at the back
        if y == 2 or floor[2][x][0] == amph[0]:
            return False
    return True


def possible_moves(floor, amph: str, pos):
    x, y = pos
    moves = []
    if y == 0:
        # can we go to our room?
        rx = AMPHIPOD_ROOMS[amph[0]]
        ry = 0
        # room empty?
        if floor[2][rx] == '.':
            ry = 2
        # room already contains our friend at the back?
        elif floor[2][rx][0] == amph[0] and floor[1][rx] == '.':
            ry = 1
        if ry:
            # hall free towards room?
            if not [p for p in range(min(x, rx), max(x, rx) + 1)
                    if floor[0][p] != '.']:
                moves.append((rx, ry))
    elif y == 1 or floor[1][x] == '.':
        # move out of room
        for p in range(x - 1, -1, -1):
            if p in HALL_POSITIONS:
                if floor[0][p] == '.':
                    moves.append((p, 0))
                else:
                    break
        for p in range(x + 1, 11):
            if p in HALL_POSITIONS:
                if floor[0][p] == '.':
                    moves.append((p, 0))
                else:
                    break
    return moves


def is_solved(floor: List[List[str]]):
    return ''.join([p[0] for p in floor[1] if p[0] in 'ABCD']) == 'ABCD' \
           and ''.join([p[0] for p in floor[2] if p[0] in 'ABCD']) == 'ABCD'


def solve(floor, positions, energy, minimum, dejavu):
    # print(f'Dejavu = {len(dejavu)}')
    if is_solved(floor):
        print(f'Found {energy} - minimum before {minimum}')
        return min(minimum, energy)
    for pos in positions:
        x, y = pos
        amph = amphipod(floor, pos)
        if needs_to_move(floor, amph, pos):
            floor[y][x] = '.'  # move out
            moves = possible_moves(floor, amph, pos)
            for npos in moves:
                nx, ny = npos
                floor[ny][nx] = amph
                new_energy = energy + needed_energy(pos, npos, amph)
                key = ''.join([''.join(l) for l in floor])
                if key not in dejavu or dejavu[key] > new_energy:
                    dejavu[key] = new_energy
                    new_positions = positions.copy()
                    new_positions.remove(pos)
                    new_positions.add(npos)
                    e = solve(floor, new_positions, new_energy, minimum, dejavu)
                    minimum = min(e, minimum)
                floor[ny][nx] = '.'
            floor[y][x] = amph  # and back in
    return minimum


def main():
    floor = [list('...........'),
             # ['.', '.', 'B1', '.', 'C1', '.', 'B2', '.', 'D2', '.', '.'],
             # ['.', '.', 'A1', '.', 'D1', '.', 'C2', '.', 'A2', '.', '.']
             ['.', '.', 'A1', '.', 'D1', '.', 'B1', '.', 'D2', '.', '.'],
             ['.', '.', 'B2', '.', 'C1', '.', 'A2', '.', 'C2', '.', '.']
             ]
    positions = {(2 * x, y + 1) for x in range(1, 5) for y in range(2)}
    energy = solve(floor, positions, 0, sys.maxsize, {})
    print(f'Total energy used = {energy}')
    assert energy == 12240


if __name__ == "__main__":
    main()
