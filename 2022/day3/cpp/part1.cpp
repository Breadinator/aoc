#include <fstream>
#include <string>
#include <vector>
#include "part1.h"
#include "common.h"

int handleLine(std::string *line) {
    int compSize = line->size() / 2;
    std::vector<char> a(line->c_str(), line->c_str() + compSize);
    std::vector<char> b(line->c_str() + compSize, line->c_str() + compSize + compSize);
    char ch = getCommonCharacter({a, b});
    return getCharPriority(ch);
}

int part1Solve(std::ifstream *file) {
    int acc = 0;
    std::string line;
    while (std::getline(*file, line)) {
        acc += handleLine(&line);
    }
    return acc;
}
