#!/usr/bin/env python3

from util import *


def count_increases(data: List[int], window: int) -> int:
    cnt = 0
    prev = sum(data[0:window])
    for i in range(window, len(data)):
        new = prev + data[i] - data[i - window]
        cnt += 1 if new > prev else 0
        prev = new
    return cnt


if __name__ == "__main__":
    measures = load('day1.txt', mapper=int)

    increases = count_increases(measures, 1)
    assert increases == 1722
    print("Part 1: 0-window increases = %d" % increases)

    increases = count_increases(measures, 3)
    assert increases == 1748
    print("Part 1: 3-window increases = %d" % increases)
