#!/usr/bin/env python3
import sys
from collections import Counter
from statistics import mean, median_low
from typing import List, Dict, Tuple

from util import load


def fuel_needed1(positions: Dict[int, int], position: int) -> int:
    fuel = 0
    for p, cnt in positions.items():
        fuel += cnt * abs(position - p)
    return fuel


def split_positions(positions: Dict[int, int], pos: int) -> \
        Tuple[int, int, int]:
    before_cnt = 0
    on_cnt = 0
    after_cnt = 0
    for p, cnt in positions.items():
        if p < pos:
            before_cnt += cnt
        elif p > pos:
            after_cnt += cnt
        else:
            on_cnt = cnt
    return before_cnt, on_cnt, after_cnt


def part1(data: List[int]) -> int:
    grouped_data = Counter(data)
    position = median_low(data)

    prev_fuel = sys.maxsize
    while True:
        fuel = fuel_needed1(grouped_data, position)
        before_cnt, on_cnt, after_cnt = split_positions(grouped_data, position)
        if before_cnt + on_cnt - after_cnt > 0:
            position += 1
        elif after_cnt + on_cnt - before_cnt > 0:
            position -= 1
        if fuel < prev_fuel:
            prev_fuel = fuel
        else:
            break
    return prev_fuel


def fuel_needed2(positions: Dict[int, int], position: int) -> int:
    fuel = 0
    for p, cnt in positions.items():
        distance = abs(position - p)
        fuel_consumption = distance * (distance + 1) // 2
        fuel += cnt * fuel_consumption
    return fuel


def part2(data: List[int]) -> int:
    grouped_data = Counter(data)
    position = round(mean(data))

    not_moved = True
    fuel = fuel_needed2(grouped_data, position)
    while True:
        fuel_before = fuel_needed2(grouped_data, position - 1)
        if fuel_before < fuel:
            fuel = fuel_before
            position -= 1
            not_moved = False
        else:
            break
    while not_moved:
        fuel_after = fuel_needed2(grouped_data, position + 1)
        if fuel_after < fuel:
            fuel = fuel_after
            position += 1
        else:
            break
    return fuel


if __name__ == "__main__":
    distances = [int(v) for v in load('day7.txt')[0].split(',')]

    needed_fuel = part1(distances)
    print("Fuel needed = %d" % needed_fuel)
    assert needed_fuel == 343468

    needed_fuel = part2(distances)
    print("Fuel needed = %d" % needed_fuel)
    assert needed_fuel == 96086265
