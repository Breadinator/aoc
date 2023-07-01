from collections.abc import Iterable
from aoc22d3 import solve_iter

def run_tests():
    test(
        1,
        157,
        [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ]
    )

def test(test_number: int, expected: int, inputs: Iterable[str]):
    assert solve_iter(inputs) == expected
    print(f"Passed test {test_number}")

if __name__ == "__main__":
    run_tests()
