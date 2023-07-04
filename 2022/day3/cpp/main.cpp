#include <iostream>
#include <string>
#include "common.h"
#include "part1.h"
#include "part2.h"

int main(int argc, char *argv[]) {
    char *path = getPath(argc, argv);
    int part = getPart(argc, argv);

    std::ifstream file(path);
    if (!file.is_open()) {
        std::cout << "Couldn't open file at " << path << std::endl;
        return 1;
    }

    int answer = part == 1 ? part1Solve(&file) : part2Solve(&file);
    std::cout << std::to_string(answer) << std::endl;

    file.close();
    return 0;
}
