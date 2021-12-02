from typing import List, TypeVar, Callable

T = TypeVar('T')

Mapper = Callable[[str], T]


def load(filename, mapper: Mapper = lambda x: x) -> List[T]:
    with open('data/' + filename, 'r') as f:
        lines = f.read().splitlines()
    return list(map(mapper, lines))
