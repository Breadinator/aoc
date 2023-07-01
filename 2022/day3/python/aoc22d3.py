from collections.abc import Iterable
from io import TextIOWrapper
import pathlib

def main():
    answer = -1
    path = get_path()
    with open(path, 'r', encoding='utf-8') as file:
        answer = solve_file(file)
        file.close()
    print(answer)


def get_path():
    return pathlib.Path(__file__).parent.parent.resolve() / "input.txt"

def solve_file(file: TextIOWrapper) -> int:
    return solve_iter(yield_lines(file))

def solve_iter(iterable: Iterable[str]) -> int:
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

if __name__ == "__main__":
    main()
