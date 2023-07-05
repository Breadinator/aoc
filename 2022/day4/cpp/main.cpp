#include <iostream>
#include <string>
#include <fstream>
#include "common.h"
#include "part1.h"
#include "part2.h"

int main(int argc, char *argv[]) {
    int part = getPart(argc, argv);
    char *path = getPath(argc, argv);

    std::ifstream file(path != nullptr ? path : "../input.txt");
    if (!file.is_open()) {
        std::cout << "Couldn't open file at " << path << std::endl;
        return 1;
    }

    int (*lineHandler)(Pairing) = part == 1 ? part1HandleLine : part2HandleLine;
    int acc = 0;
    std::string line;
    while (std::getline(file, line)) {
        Pairing pair = parse(line);
        acc += (*lineHandler)(pair);
    }
    std::cout << std::to_string(acc) << std::endl;

    file.close();
    return 0;
}
