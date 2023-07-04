#include <fstream>
#include <string>
#include <vector>
#include "part2.h"
#include "common.h"

int handleLines(std::string lines[3]) {
    std::vector<char> a(lines[0].c_str(), lines[0].c_str() + lines[0].size());
    std::vector<char> b(lines[1].c_str(), lines[1].c_str() + lines[1].size());
    std::vector<char> c(lines[2].c_str(), lines[2].c_str() + lines[2].size());
    char ch = getCommonCharacter({a, b, c});
    return getCharPriority(ch);
}

int part2Solve(std::ifstream *file) {
    int acc = 0;
    std::string line;
    std::string lines[3];
    int count = 0;
    while (std::getline(*file, line)) {
        lines[count++] = line;
        if (count == 3) {
            acc += handleLines(lines);
            count = 0;
        }
    }
    return acc;
}
