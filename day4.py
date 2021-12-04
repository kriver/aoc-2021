#!/usr/bin/env python3
from typing import Dict, Tuple

from util import *


class Board:
    SIZE = 5

    def __init__(self):
        self._numbers: Dict[int, Tuple[int, int]] = {}
        self._rows: List[int] = []
        self._columns: List[int] = []

    def add_row(self, data: List[int]):
        self._rows.append(0)
        self._columns = [0] * len(data)
        for i, n in enumerate(data):
            self._numbers[n] = (i, len(self._rows) - 1)

    def draw_number(self, number: int) -> int:
        score = 0
        if number in self._numbers:
            (x, y) = self._numbers[number]
            del self._numbers[number]
            self._columns[x] += 1
            self._rows[y] += 1
            if self._columns[x] == self.SIZE or self._rows[y] == self.SIZE:
                score = number * sum(self._numbers.keys())
        return score


def to_int(lines: List[str]) -> List[int]:
    return list(map(int, lines))


def parse_boards(lines: List[str]) -> Dict[int, Board]:
    all_boards = {}
    num = 0
    board = None
    for line in lines:
        if len(line) == 0:
            num += 1
            board = Board()
            all_boards[num] = board
        else:
            board.add_row(to_int(line.split()))
    return all_boards


def first_win(all_boards: Dict[int, Board], drawn_numbers: List[int]) -> int:
    for n in drawn_numbers:
        for b in all_boards.values():
            score = b.draw_number(n)
            if score != 0:
                return score
    return 0


def last_win(all_boards: Dict[int, Board], drawn_numbers: List[int]) -> int:
    for n in drawn_numbers:
        for board_id in list(all_boards.keys()):
            b = all_boards[board_id]
            score = b.draw_number(n)
            if score != 0:
                del all_boards[board_id]
                if len(all_boards) == 0:
                    return score
    return 0


if __name__ == "__main__":
    input_lines = load('day4.txt')

    numbers = [int(n) for n in input_lines[0].split(',')]
    boards = parse_boards(input_lines[1:])
    final_score = first_win(boards, numbers)
    assert final_score == 63424
    print("First win score = %d" % final_score)

    final_score = last_win(boards, numbers)
    assert final_score == 23541
    print("Last win score = %d" % final_score)
