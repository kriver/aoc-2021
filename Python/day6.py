#!/usr/bin/env python3
from collections import Counter
from typing import List

from util import load


def count_fish(fish: List[int], days: int) -> int:
    grouped = Counter(fish)
    for d in range(days):
        new_grouped = {}
        for days, cnt in grouped.items():
            if days == 0:
                days = 6
                new_grouped[8] = cnt
            else:
                days -= 1
            if days in new_grouped:
                new_grouped[days] = new_grouped[days] + cnt
            else:
                new_grouped[days] = cnt
        grouped = new_grouped
    return sum(grouped.values())


if __name__ == "__main__":
    fish_data = [int(v) for v in load('day6.txt')[0].split(',')]

    num = count_fish([3, 4, 3, 1, 2], 18)
    print("Test fish count after 18 days = %d" % num)
    assert num == 26
    num = count_fish([3, 4, 3, 1, 2], 80)
    print("Test fish count after 80 days = %d" % num)
    assert num == 5934
    num = count_fish([3, 4, 3, 1, 2], 256)
    print("Test fish count after 256 days = %d" % num)
    assert num == 26984457539

    num = count_fish(fish_data, 80)
    print("Fish count after 80 days = %d" % num)
    assert num == 345387
    num = count_fish(fish_data, 256)
    print("Fish count after 256 days = %d" % num)
    assert num == 1574445493136
