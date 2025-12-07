#include <string>
#include <sstream>
#include <fstream>
#include <vector>
#include <iostream>

class Rotation {
    public:
        bool clockwise;
        int amount;
};

std::vector<Rotation> parseLines() {
    std::vector<Rotation> output;
    std::ifstream input;
    input.open("../input.txt");

    std::string line;
    while (std::getline(input, line)) {
        bool clockwise;
        if (line[0] == 'R') {
            clockwise = true;
        } else {
            clockwise = false;
        }
        int amount = std::stoi(line.substr(1));

        output.push_back(Rotation {
            .clockwise = clockwise,
            .amount = amount,
        });
    }

    input.close();
    return output;
}

static int mod_positive(int x, int m) {
    int r = x % m;
    if (r < 0) r += m;
    return r;
}

int part1(std::vector<Rotation> rotations) {
    int value = 50;
    int times_zero = 0;
    for (Rotation r : rotations) {
        if (r.clockwise) {
            value += r.amount;
        } else {
            value -= r.amount;
        }
        value = mod_positive(value, 100);
        if (value == 0) {
            times_zero++;
        }
    }
    return times_zero;
}

int part2(std::vector<Rotation> rotations) {
    int value = 50;
    int times_zero = 0;

    for (Rotation r : rotations) {
        int a = r.amount;
        if (r.clockwise) {
            if (value == 0) {
                times_zero += a / 100;
            } else {
                int threshold = 100 - value;
                if (a >= threshold) {
                    times_zero += ((a - threshold) / 100) + 1;
                }
            }
            value = mod_positive(value + a, 100);
        } else {
            if (value == 0) {
                times_zero += a / 100;
            } else {
                int threshold = value;
                if (a >= threshold) {
                    times_zero += ((a - threshold) / 100) + 1;
                }
            }
            value = mod_positive(value - a, 100);
        }
    }

    return times_zero;
}

int main() {
    std::vector<Rotation> rotations = parseLines();

    printf("%i\n", part1(rotations));
    printf("%i\n", part2(rotations));

    return 0;
}