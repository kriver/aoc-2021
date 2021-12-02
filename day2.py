#!/usr/bin/env python3
import re
from typing import Tuple

from util import *

LINE_REGEX = re.compile(r'^(\w+) (\d+)$')
Line = Tuple[str, int]


def map_line(line: str) -> Line:
    m = re.match(LINE_REGEX, line)
    if not m:
        raise Exception('invalid input: ' + line)
    return m.group(1), int(m.group(2))


def part1(lines: List[Line]) -> int:
    (pos, depth) = (0, 0)
    for line in lines:
        (direction, value) = line
        if direction == 'forward':
            pos += value
        else:
            depth += value if direction == 'down' else -value
    return pos * depth


def part2(lines: List[Line]) -> int:
    (pos, depth, aim) = (0, 0, 0)
    for line in lines:
        (direction, value) = line
        if direction == 'forward':
            pos += value
            depth += aim * value
        else:
            aim += value if direction == 'down' else -value
    return pos * depth


if __name__ == "__main__":
    course = load('day2.txt', mapper=map_line)
    result = part1(course)
    assert result == 1561344
    print("Part 1: %d" % result)

    result = part2(course)
    assert result == 1848454425
    print("Part 2: %d" % result)
