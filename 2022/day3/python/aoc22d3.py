from collections.abc import Iterable
from io import TextIOWrapper
import pathlib
import sys

def main():
    part = sys.argv[1] if len(sys.argv) != 1 else None
    path = get_path()

    if part == "1" or part is None:
        with open(path, 'r', encoding='utf-8') as file:
            print(part1_solve_file(file))
            file.close()
    if part == "2" or part is None:
        with open(path, 'r', encoding='utf-8') as file:
            print(part2_solve_file(file))
            file.close()


def get_path():
    return pathlib.Path(__file__).parent.parent.resolve() / "input.txt"

def part1_solve_file(file: TextIOWrapper) -> int:
    return part1_solve_iter(yield_lines(file))

def part1_solve_iter(iterable: Iterable[str]) -> int:
    acc = 0
    for bag in iterable:
        acc += get_set_priority(get_common_elements(bag))
    return acc

def yield_lines(file: TextIOWrapper) -> Iterable[str]:
    while True:
        line = file.readline()
        if line == "": # input has no new-lines
            break
        yield line[:-1] # removes \n from the end

def get_priority(char: str) -> int:
    ch_val = ord(char)
    return ch_val - (38 if ch_val < 97 else 96)

def get_set_priority(chars: set[str]) -> int:
    acc = 0
    for char in chars:
        acc += get_priority(char)
    return acc

def get_common_elements(bag: str) -> set[str]:
    comp_len = len(bag) // 2
    return {x for x in bag[:comp_len] if x in bag[comp_len:]}

def part2_solve_file(file: TextIOWrapper) -> int:
    return part2_solve_iter(yield_lines(file))

def part2_solve_iter(iterable: Iterable[str]) -> int:
    acc = 0
    while True:
        try:
            block: tuple[str, str, str] = (next(iterable), next(iterable), next(iterable))
            # might be better to use a hashset? idk what python uses for sets under the hood
            common = {x for x in block[0] if x in block[1] and x in block[2]}
            acc += get_priority(common.pop())
        except StopIteration:
            break
    return acc

if __name__ == "__main__":
    main()
