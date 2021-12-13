#!/usr/bin/env python3
import queue
from statistics import median

from util import *

CORRUPTION = {')': 3, ']': 57, '}': 1197, '>': 25137}
FIXING = {')': 1, ']': 2, '}': 3, '>': 4}

CLOSING = {'(': ')', '[': ']', '{': '}', '<': '>'}


def calc_score(line: str) -> int:
    expected = queue.LifoQueue()
    for c in line:
        if c in ['(', '[', '{', '<']:
            expected.put(CLOSING[c])
        else:
            last = expected.get()
            if c != last:
                return -CORRUPTION[c]  # negative for corruption
    fixing_str = ''
    while not expected.empty():
        fixing_str += str(FIXING[expected.get()])
    return int(fixing_str, 5)


def calc_corruption_score(scores: List[int]) -> int:
    corrupted = filter(lambda s: s < 0, scores)
    return sum(-s for s in corrupted)


def calc_fixing_score(scores: List[int]) -> int:
    fixed = filter(lambda s: s > 0, scores)
    return median(fixed)


if __name__ == "__main__":
    data = load('day10.txt')
    data_scores = [calc_score(line) for line in data]

    score = calc_corruption_score(data_scores)
    print('Corruption score: %d' % score)
    assert score == 318099

    score = calc_fixing_score(data_scores)
    print('Fixing score: %d' % score)
    assert score == 2389738699
