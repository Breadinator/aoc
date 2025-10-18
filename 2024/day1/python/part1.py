from functools import reduce
from operator import add


def main():
    left, right = parse_input_and_sort()
    result = find_distance(left, right)
    print(result, end="")


def parse_input_and_sort() -> tuple[list[int], list[int]]:
    left: list[int] = []
    right: list[int] = []
    with open('../input.txt', 'r') as file:
        for line in file:
            [line_left, line_right] = line.split()
            left.append(int(line_left))
            right.append(int(line_right))
    left.sort()
    right.sort()
    return (left, right)


def find_distance(left: list[int], right: list[int]) -> int:
    return reduce(add, map(lambda x: abs(x[0]-x[1]), zip(left, right)))


if __name__ == "__main__":
    main()