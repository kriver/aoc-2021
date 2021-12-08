#!/usr/bin/env python3
import re
from typing import Tuple, Dict

from util import *

LINE_REGEX = re.compile(r'^(.+) \| (.+)$')
Line = Tuple[List[str], List[str]]


def map_line(line: str) -> Line:
    m = re.match(LINE_REGEX, line)
    if not m:
        raise Exception('invalid input: ' + line)
    return m.group(1).split(), m.group(2).split()


def part1(lines: List[Line]) -> int:
    cnt = 0
    for line in lines:
        for output in line[1]:
            l = len(output)
            if l in [2, 3, 4, 7]:
                cnt += 1
    return cnt


def sort_str(s: str) -> str:
    return ''.join(sorted(s))


def overlap(a: str, b: str) -> int:
    set_a = set(a)
    set_b = set(b)
    return len(set_a.intersection(set_b))


# 0: remainder (out of length 6)
# 1: 2 segments
# 2: overlap with 4 is 2 (out of length 5)
# 3: overlap with 1 is 2 (out of length 5)
# 4: 4 segments
# 5: overlap with 4 is 3 (out of length 5)
# 6: overlap with 7 is 2 (out of length 6)
# 7: 3 segments
# 8: 7 segments
# 9: overlap with 5 is 5 (out of length 6)
def decode_input(line: Line) -> Dict[int, str]:
    encoded = line[0]
    decoded = {}
    while len(encoded) > 1:
        for e in list(encoded):
            if len(e) == 2:
                decoded[1] = e
            elif len(e) == 3:
                decoded[7] = e
            elif len(e) == 4:
                decoded[4] = e
            elif len(e) == 7:
                decoded[8] = e
            elif len(e) == 5:
                if 1 in decoded and overlap(decoded[1], e) == 2:
                    decoded[3] = e
                elif 4 in decoded and 3 in decoded:
                    if overlap(decoded[4], e) == 2:
                        decoded[2] = e
                    elif overlap(decoded[4], e) == 3:
                        decoded[5] = e
                    else:
                        continue
                else:
                    continue
            elif len(e) == 6:
                if 7 in decoded and overlap(decoded[7], e) == 2:
                    decoded[6] = e
                elif 5 in decoded and overlap(decoded[5], e) == 5:
                    decoded[9] = e
                else:
                    continue
            else:
                continue
            encoded.remove(e)
    decoded[0] = encoded[0]
    return decoded


def decode(line: Line) -> int:
    decoded = decode_input(line)
    inverse = {sort_str(v): k for k, v in decoded.items()}
    num = 0
    for d in line[1]:
        num = num * 10 + inverse[sort_str(d)]
    return num


def decode_all(lines: List[Line]) -> int:
    return sum([decode(line) for line in lines])


if __name__ == "__main__":
    parsed_data = load('day8.txt', mapper=map_line)

    result = part1(parsed_data)
    print('Number of 1, 4, 7, 8 in output: %d' % result)
    assert result == 310

    result = decode_all(parsed_data)
    print('Decoded sum: %d' % result)
    assert result == 915941
