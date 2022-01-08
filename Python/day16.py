#!/usr/bin/env python3
import numpy

from util import *


def to_bits(line):
    return ''.join([format(int(c, 16), "04b") for c in line])


def parse_literal(msg, offset):
    value = ''
    while msg[offset] == '1':
        value += msg[offset + 1:offset + 5]
        offset += 5
    value += msg[offset + 1:offset + 5]
    return offset + 5, int(value, 2)


def parse_sub_by_bits(msg, offset, length):
    packets = []
    while offset < length:
        offset, value = parse_packet(msg, offset)
        packets.append(value)
    return offset, packets


def parse_sub_by_numb(msg, offset, length):
    packets = []
    while len(packets) < length:
        offset, value = parse_packet(msg, offset)
        packets.append(value)
    return offset, packets


def parse_packet(msg, offset):
    version = int(msg[offset:offset + 3], 2)
    ptype = int(msg[offset + 3:offset + 6], 2)
    offset += 6
    if ptype == 4:
        offset, value = parse_literal(msg, offset)
    elif msg[offset] == '0':
        length = int(msg[offset + 1:offset + 16], 2)
        offset += 16
        offset, value = parse_sub_by_bits(msg, offset, offset + length)
    else:
        length = int(msg[offset + 1:offset + 12], 2)
        offset += 12
        offset, value = parse_sub_by_numb(msg, offset, length)
    return offset, (version, ptype, value)


def parse(msg):
    offset, msg = parse_packet(msg, 0)
    return msg


def sum_versions(messages):
    total = 0
    for msg in messages:
        total += msg[0]
        if msg[1] != 4:
            total += sum_versions(msg[2])
    return total


def calculate_1(msg):
    ptype = msg[1]
    sub = msg[2]
    if ptype == 4:
        return sub
    subval = calculate(sub)
    if ptype == 0:
        value = numpy.sum(subval)
    elif ptype == 1:
        value = numpy.prod(subval)
    elif ptype == 2:
        value = min(subval)
    elif ptype == 3:
        value = max(subval)
    elif ptype == 5:
        value = 1 if subval[0] > subval[1] else 0
    elif ptype == 6:
        value = 1 if subval[0] < subval[1] else 0
    elif ptype == 7:
        value = 1 if subval[0] == subval[1] else 0
    else:
        raise "oops"
    return value


def calculate(messages):
    return [calculate_1(m) for m in messages]


if __name__ == "__main__":
    data = load('day16.txt')[0]
    bits = to_bits(data)
    message = parse(bits)

    version_sum = sum_versions([message])
    print('Version sum: %d' % version_sum)
    assert version_sum == 879

    result = calculate([message])
    print(f'Result: {result}')
    assert result[0] == 539051801941
