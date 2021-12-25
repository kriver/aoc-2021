#!/usr/bin/env python3
import sys
from typing import List

from util import load


def run_program(lines: List[str], inp: List[int]):
    def getval(regs, rhs):
        try:
            return int(rhs)
        except ValueError:
            return regs[rhs]

    regs = {'w': 0, 'x': 0, 'y': 0, 'z': 0}
    idx = 0
    for line in lines:
        l = line.split()
        op = l[0]
        lhs = l[1]
        if op == 'inp':
            regs[lhs] = inp[idx]
            idx += 1
        else:
            rhs = getval(regs, l[2])
            if op == 'add':
                regs[lhs] += rhs
            elif op == 'mul':
                regs[lhs] *= rhs
            elif op == 'div':
                if rhs != 0:
                    regs[lhs] //= rhs
                else:
                    return -1
            elif op == 'mod':
                if regs[lhs] >= 0 and rhs > 0:
                    regs[lhs] %= rhs
                else:
                    return -1
            elif op == 'eql':
                regs[lhs] = 1 if regs[lhs] == rhs else 0
    return regs['z']


def run_python(inp: List[int]):
    x_add = [13, 12, 11, 0, 15, -13, 10, -9, 11, 13, -14, -3, -2, -14]
    z_div = [1, 1, 1, 26, 1, 26, 1, 26, 1, 1, 26, 26, 26, 26]
    y_add = [14, 8, 5, 4, 10, 13, 16, 5, 6, 13, 6, 7, 13, 3]
    z = 0
    for i, digit in enumerate(inp):
        if z % 26 == digit - x_add[i]:  # can only be true for i in [3,5,7,10,11,12,13]
            z //= 26
        else:
            z //= z_div[i]
            z = z * 26 + digit + y_add[i]
    return z


def solve(data, inp, start):
    if start >= 14:  # nothing to solve, still calculate
        return inp, run_python(inp)

    minimum = sys.maxsize
    for i in range(start, 14):
        minimum = sys.maxsize
        minimum_x = -1
        for x in range(9, 0, -1):
            inp[i] = x
            s = run_python(inp)
            # sp = run_program(data, inp)
            # assert s == sp
            if s < minimum:
                minimum = s
                minimum_x = x
        inp[i] = minimum_x
        # print(f'inp[{i}] = {x} -> {inp} = {run_python(inp)}')
    return inp, minimum


def main():
    data = load('day24.txt')
    model_number = ''

    # 11499629198471 if keeping minimum per iteration
    # 91499629198479 but also this
    nines = [int(c) for c in '99999999999999']
    inp = list(nines)
    sol, minimum = solve(data, inp, 0)
    for i in range(14):
        # try increasing this digit
        for d in range(sol[i] + 1, 10):
            inp = sol[0:i + 1] + nines[i + 1:]
            inp[i] = d
            inp, maybe_min = solve(data, inp, i + 1)
            if maybe_min <= minimum:
                sol[i] = d
                minimum = maybe_min
        model_number = ''.join([str(i) for i in sol])
        # print(f'Model number = {model_number} with {run_python(sol)}')
    print(f'Model number = {model_number} with {run_python(sol)}')
    assert model_number == '93499629698999'


def brute_force(lines: List[str]):
    for n in range(99999999999999, 11111111111111, -1):
        s = str(n)
        if s.find('0') != -1:
            continue
        inp = [int(c) for c in s]
        # outp = run_program(lines, inp)
        outp = run_python(inp)
        print(f'{n} -> {outp}')
        if outp == 0:
            break
    print(f'Found {n}')


if __name__ == "__main__":
    # brute_force(None)
    main()