#!/usr/bin/env python3
from util import load


def parse(line):
    depth = -1
    num = []
    for c in line:
        if c == '[':
            depth += 1
        elif c == ']':
            depth -= 1
        elif c != ',':
            num.append((int(c), depth))
    return num


def add(num1, num2):
    return [(n, d + 1) for (n, d) in num1] + [(n, d + 1) for (n, d) in num2]


def explode(num):
    idx = next((i for i, (n, d) in enumerate(num) if d == 4), -1)
    if idx >= 0:
        # left
        n, _ = num[idx]
        if idx > 0:
            nl, dl = num[idx - 1]
            num[idx - 1] = nl + n, dl
        # right
        n, _ = num[idx + 1]
        if idx < len(num) - 2:
            nr, dr = num[idx + 2]
            num[idx + 2] = nr + n, dr
        # self
        num[idx:idx + 2] = [(0, 3)]
    return num


def split(num):
    idx = next((i for i, (n, d) in enumerate(num) if n > 9), -1)
    if idx >= 0:
        n, d = num[idx]
        half = n // 2
        num[idx:idx + 1] = [(half, d + 1), (n - half, d + 1)]
    return num


def add_reduce(num1, num2):
    num = add(num1, num2)
    while True:
        length = len(num)
        num = explode(num)
        if length != len(num):
            continue
        num = split(num)
        if length != len(num):
            continue
        break
    return num


def to_tuples(num):
    while len(num) > 1:
        idx = 0
        while idx + 1 < len(num):
            n1, d1 = num[idx]
            n2, d2 = num[idx + 1]
            if d1 == d2:
                num[idx:idx + 2] = [((n1, n2), d1 - 1)]
                break
            idx += 1
    return num[0][0]


def magnitude_r(left, right):
    if type(left) != int:
        left = magnitude(left)
    if type(right) != int:
        right = magnitude(right)
    return 3 * left + 2 * right


def magnitude(num):
    left, right = num
    return magnitude_r(left, right)


def do_homework(lines):
    lhs = parse(lines[0])
    for line in lines[1:]:
        rhs = parse(line)
        lhs = add_reduce(lhs, rhs)
    return magnitude(to_tuples(lhs))


def magnitude_sum(num1, num2):
    return magnitude(to_tuples(add_reduce(num1, num2)))


def do_backside(lines):
    numbers = [parse(line) for line in lines]
    largest = 0
    for i in range(len(numbers)):
        for j in range(i + 1, len(numbers)):
            largest = max(largest, magnitude_sum(numbers[i], numbers[j]))
            largest = max(largest, magnitude_sum(numbers[j], numbers[i]))
    return largest


if __name__ == "__main__":
    assert parse('[[[[[9,8],1],2],3],4]') == [(9, 4), (8, 4), (1, 3), (2, 2), (3, 1), (4, 0)]
    assert add(parse('[1,2]'), parse('[[3,4],5]')) == [(1, 1), (2, 1), (3, 2), (4, 2), (5, 1)]
    assert to_tuples(explode(parse('[[[[[9,8],1],2],3],4]'))) == ((((0, 9), 2), 3), 4)
    assert to_tuples(explode(parse('[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]'))) == \
           ((3, (2, (8, 0))), (9, (5, (4, (3, 2)))))
    assert to_tuples(split([(15, 0), (13, 0)])) == ((7, 8), 13)
    assert to_tuples(add_reduce(parse('[[[[4,3],4],4],[7,[[8,4],9]]]'), parse('[1,1]'))) == \
           ((((0, 7), 4), ((7, 8), (6, 0))), (8, 1))
    assert magnitude(to_tuples(parse('[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]'))) \
           == 4140

    data = load('day18.txt')

    mag = do_homework(data)
    print('Magnitude: %d' % mag)
    assert mag == 2907

    mag = do_backside(data)
    print('Largest magnitude: %d' % mag)
    assert mag == 4690
