#!/usr/bin/env python3
import math

(X1, Y1) = (60, -171)
(X2, Y2) = (94, -136)


# Sample area
# (X1, Y1) = (20, -10)
# (X2, Y2) = (30, -5)


def move(pos, velocity):
    x, y = pos
    vx, vy = velocity
    x = x + vx
    y = y + vy
    vx = max(0, vx - 1)
    vy -= 1
    return (x, y), (vx, vy)


def in_area(pos):
    x, y = pos
    return X1 <= x <= X2 and Y1 <= y <= Y2


def overshot(pos):
    x, y = pos
    return x > max(X1, X2) or y < min(Y1, Y2)


def move_until_done(velocity):
    pos = 0, 0
    highest = 0
    step = 0

    while True:
        if in_area(pos):
            return highest
        elif overshot(pos):
            return -1
        else:
            (x, y), velocity = move(pos, velocity)
            if y > highest:
                highest = y
            pos = (x, y)
        step += 1


# minimum vx: vx+(vx-1)+...+2+1+0 = (vx+1)*(vx/2) >= X1, vx > 10.46
def vx_for(x):
    return int(math.ceil((-1 + math.sqrt(1 + 8 * x)) / 2))


def find_highest():
    # 1) As long as we reach the area, the X speed is irrelevant for highest
    #    point.
    # 2) When shooting up, at some point Y coordinate will be 0 again. At that
    #    time the next VY is the initial -(VY + 1). So we can limit our VY
    #    search range to -(Y2 + 1) and -(Y1 + 1). Lower values won't go as high.
    #    Higher values will overshoot.
    vx, vy = vx_for(X1), -(Y2 + 1)
    highest = 0
    while True:
        h = move_until_done((vx, vy))
        if h >= highest:
            highest = h
        elif vy > abs(Y1) + 1:
            break
        vy += 1
    return highest


def find_all():
    velocities = []
    for vy in range(Y1 - 1, -Y1 + 1):
        for vx in range(vx_for(X1), X2 + 1):
            v = (vx, vy)
            if move_until_done((vx, vy)) >= 0:
                velocities.append(v)
    return velocities


if __name__ == "__main__":
    peak = find_highest()
    print('Highest position: %d' % peak)
    assert peak == 14535

    all_v = find_all()
    print('Number of velocities: %d' % len(all_v))
    assert len(all_v) == 2270
