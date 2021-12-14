#!/usr/bin/env python3
from collections import Counter
from typing import Tuple, Dict

from util import *

Rules = Dict[str, str]
Pairs = Dict[str, int]
Elements = Dict[str, int]
Data = Tuple[Elements, Pairs, Rules]


def parse_pairs(line: str) -> Pairs:
    pairs = [pair[0] + pair[1] for pair in zip(line[:-1], line[1:])]
    return Counter(pairs)


def parse_rules(lines: List[str]) -> Rules:
    rules = {}
    for line in lines:
        kv = line.split(' -> ')
        rules[kv[0]] = kv[1]
    return rules


def parse(lines: List[str]) -> Data:
    return Counter(list(lines[0])), \
           parse_pairs(lines[0]), \
           parse_rules(lines[2:])


def evolve(data: Data) -> Data:
    elements, pairs, rules = data
    new_pairs = {}
    new_elements = dict(elements)
    for p, cnt in pairs.items():
        c = rules[p]
        p1, p2 = p[0] + c, c + p[1]
        new_pairs[p1] = new_pairs.get(p1, 0) + cnt
        new_pairs[p2] = new_pairs.get(p2, 0) + cnt
        new_elements[c] = new_elements.get(c, 0) + cnt
    return new_elements, new_pairs, rules


def evolve_n(data: Data, n: int) -> Data:
    for i in range(n):
        data = evolve(data)
    return data


if __name__ == "__main__":
    initial_data = parse(load('day14.txt'))

    result_data = evolve_n(initial_data, 10)
    sorted_result = sorted(result_data[0].values())
    diff = sorted_result[-1] - sorted_result[0]
    print("Quantity diff after 10 steps: %d" % diff)
    assert diff == 2657

    result_data = evolve_n(initial_data, 40)
    sorted_result = sorted(result_data[0].values())
    diff = sorted_result[-1] - sorted_result[0]
    print("Quantity diff after 40 steps: %d" % diff)
    assert diff == 2911561572630
