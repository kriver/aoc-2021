#!/usr/bin/env python3
import re
from typing import Dict, Optional

from util import *

LINE_REGEX = re.compile(r'^(\w+)-(\w+)$')
Edges = Dict[str, List[str]]


def map_line(line):
    m = re.match(LINE_REGEX, line)
    if not m:
        raise Exception('invalid input: ' + line)
    return m.group(1), m.group(2)


def lines_to_edges(lines) -> Edges:
    edges = {}
    for a, b in lines:
        if b != 'start':
            edges.setdefault(a, []).append(b)
        if a != 'start':
            edges.setdefault(b, []).append(a)
    return edges


def find_paths_r1(edges: Edges, current_cave: str, current_path: List[str], all_paths: List[List[str]]):
    for cave in edges[current_cave]:
        if cave == 'end':
            all_paths.append(current_path + [cave])
        else:
            if cave.islower() and cave in current_path:
                continue
            find_paths_r1(edges, cave, current_path + [cave], all_paths)


def find_paths_1(edges):
    cave = 'start'
    current = [cave]
    all_paths = []
    find_paths_r1(edges, cave, current, all_paths)
    return all_paths


def find_paths_r2(edges: Edges, current_cave: str, twice: Optional[str],
                  current_path: List[str], all_paths: List[List[str]]):
    for cave in edges[current_cave]:
        if cave == 'end':
            all_paths.append(current_path + [cave])
        else:
            if cave.islower() and cave in current_path:
                if twice:
                    continue
                find_paths_r2(edges, cave, cave, current_path + [cave], all_paths)
            else:
                find_paths_r2(edges, cave, twice, current_path + [cave], all_paths)


def find_paths_2(edges):
    cave = 'start'
    current = [cave]
    all_paths = []
    find_paths_r2(edges, cave, None, current, all_paths)
    return all_paths


if __name__ == "__main__":
    data = load('day12.txt', mapper=map_line)
    edge_map = lines_to_edges(data)

    paths = find_paths_1(edge_map)
    print("Number of paths: %d" % len(paths))
    assert len(paths) == 5076

    paths = find_paths_2(edge_map)
    print("Number of paths: %d" % len(paths))
    assert len(paths) == 145643
