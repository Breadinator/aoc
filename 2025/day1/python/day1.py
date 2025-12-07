from dataclasses import dataclass
from typing import Literal

@dataclass
class Rotation:
    direction: Literal['L', 'R']
    amount: int

class Cracker:
    def __init__(self, rotations: list[Rotation], start_value: int = 50) -> None:
        self.rotations = rotations
        self.start_value = start_value

    def part1(self) -> int:
        value = self.start_value
        times_zero = 0
        for rotation in self.rotations:
            if rotation.direction == 'L':
                value -= rotation.amount
            else:
                value += rotation.amount
            value %= 100
            if value == 0:
                times_zero += 1
        return times_zero

    def part2(self) -> int:
        value = self.start_value
        times_zero = 0

        for rotation in self.rotations:
            A = rotation.amount
            if rotation.direction == 'R':
                if value == 0:
                    times_zero += A // 100
                else:
                    threshold = 100 - value
                    if A >= threshold:
                        times_zero += ((A - threshold) // 100) + 1
                value = (value + A) % 100
            else:
                if value == 0:
                    times_zero += A // 100
                else:
                    threshold = value
                    if A >= threshold:
                        times_zero += ((A - threshold) // 100) + 1
                value = (value - A) % 100

        return times_zero

def parse_rotation(s: str) -> Rotation:
    if len(s) < 2:
        raise Exception(f"input of `{s}` is too short")
    d = s[0]
    if d == 'L' or d == 'R':
        direction = d
    else:
        raise Exception(f"invalid direction in `{s}`")
    
    amount = int(s[1:])
    return Rotation(direction, amount)

def main(s: str):
    rotations = [parse_rotation(line) for line in s.splitlines()]
    cracker = Cracker(rotations)
    print(cracker.part1())
    print(cracker.part2())

if __name__ == '__main__':
    with open("../input.txt", "r") as f:
        main(f.read())