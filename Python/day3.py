#!/usr/bin/env python3
import functools

from util import *


def bit_counts(lines: List[str]) -> List[int]:
    sum_counts = [int(c) for c in lines[0]]
    for line in lines[1:]:
        line_counts = [int(c) for c in line]
        sum_counts = [sum(x) for x in zip(sum_counts, line_counts)]
    return sum_counts


def power_consumption(lines: List[str]) -> int:
    counts = bit_counts(lines)
    gamma_bits = [1 if c >= len(lines) / 2 else 0 for c in counts]
    gamma_rate = functools.reduce(lambda a, b: a * 2 + b, gamma_bits)
    epsilon_rate = (2 ** (len(counts)) - 1) ^ gamma_rate
    return gamma_rate * epsilon_rate


def bit_criteria_filtering(lines: List[str],
                           select_one: Callable[[int, int], bool]) -> int:
    i = 0
    while len(lines) > 1:
        counts = bit_counts(lines)
        bit = '1' if select_one(counts[i], len(lines)) else '0'
        lines = list(filter(lambda l: l[i] == bit, lines))
        i += 1
    return int(lines[0], 2)


def life_support(lines: List[str]) -> int:
    oxygen_rating = bit_criteria_filtering(lines, lambda c, r: c >= r / 2)
    co2_scrubber_rating = bit_criteria_filtering(lines, lambda c, r: c < r / 2)
    return oxygen_rating * co2_scrubber_rating


if __name__ == "__main__":
    diagnostic = load('day3.txt')

    result = power_consumption(diagnostic)
    assert result == 4118544
    print("Power consumption = %d" % result)

    result = life_support(diagnostic)
    assert result == 3832770
    print("Life support rating = %d" % result)
