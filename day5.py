#!/usr/bin/env python3
import re
from typing import Tuple, List, Dict

from util import load

LINE_REGEX = re.compile(r'^(\d+),(\d+) -> (\d+),(\d+)$')
Coord = Tuple[int, int]


class Line:
    _X = 0
    _Y = 1

    def __init__(self, start: Coord, end: Coord):
        self._start = start
        self._end = end

    def is_horizontal(self):
        return self._start[self._Y] == self._end[self._Y]

    def is_vertical(self):
        return self._start[self._X] == self._end[self._X]

    def is_horz_or_vert(self):
        return self.is_horizontal() or self.is_vertical()

    def points(self) -> List[Coord]:
        dx = self._end[self._X] - self._start[self._X]
        dy = self._end[self._Y] - self._start[self._Y]
        steps = max(abs(dx), abs(dy))
        coords: List[Coord] = []
        for i in range(steps):
            coords.append((self._start[self._X] + i * dx // steps,
                           self._start[self._Y] + i * dy // steps))
        coords.append(self._end)
        return coords


def map_line(line: str) -> Line:
    m = re.match(LINE_REGEX, line)
    if not m:
        raise Exception('invalid input: ' + line)
    return Line((int(m.group(1)), int(m.group(2))),
                (int(m.group(3)), int(m.group(4))))


def build_map(lines: List[Line], incl_diagonal=False) -> Dict[Coord, int]:
    line_map = {}
    for line in lines:
        if incl_diagonal or line.is_horz_or_vert():
            points = line.points()
            for point in points:
                count = line_map.get(point, 0)
                line_map[point] = count + 1
    return line_map


def count_overlaps(line_map: Dict[Coord, int]) -> int:
    return len(list(filter(lambda v: v > 1, line_map.values())))


if __name__ == "__main__":
    lines_data = load('day5.txt', mapper=map_line)

    map_data = build_map(lines_data)
    overlaps = count_overlaps(map_data)
    assert overlaps == 4873
    print("Horz/vert Overlaps = %d" % overlaps)

    map_data = build_map(lines_data, True)
    overlaps = count_overlaps(map_data)
    assert overlaps == 19472
    print("All overlaps = %d" % overlaps)
