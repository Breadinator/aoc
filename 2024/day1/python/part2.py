from typing import Optional


def main():
    left, right = parse_input_and_sort()
    sum = calculate(left, right)
    print(sum, end="")


def calculate(left: list[int], right: list[int]):
    sum = 0
    current_num: Optional[int] = None
    current_sum: Optional[int] = None
    for num in left:
        if current_num and num == current_num and current_sum:
            sum += current_sum
            continue
        current_num = num
        current_sum = 0
        for num2 in right:
            if num2 == num:
                current_sum += num
        sum += current_sum
    return sum
        

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


if __name__ == "__main__":
    main()