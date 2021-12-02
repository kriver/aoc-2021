#!/usr/bin/env python3
import re

from util import *

LINE_REGEX = re.compile(r'^(\w+) (\d+)$')


def part1(lines: List[str]) -> int:
    (pos, depth) = (0, 0)
    for line in lines:
        m = re.match(LINE_REGEX, line)
        if not m:
            raise Exception('invalid input: ' + line)
        (direction, dist) = m.group(1), int(m.group(2))
        if direction == 'forward':
            pos += dist
        else:
            depth += dist if direction == 'down' else -dist
    return pos * depth


def part2(lines: List[str]) -> int:
    (pos, depth, aim) = (0, 0, 0)
    for line in lines:
        m = re.match(LINE_REGEX, line)
        if not m:
            raise Exception('invalid input: ' + line)
        (direction, value) = m.group(1), int(m.group(2))
        if direction == 'forward':
            pos += value
            depth += aim * value
        else:
            aim += value if direction == 'down' else -value
    return pos * depth


if __name__ == "__main__":
    course = load('day2.txt')
    result = part1(course)
    assert result == 1561344
    print("Part 1: %d" % result)

    result = part2(course)
    print("Part 2: %d" % result)
