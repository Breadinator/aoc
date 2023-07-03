#include <fstream>
#include <iostream>
#include <string>
#include "lib.h"

enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
};

enum Result {
    Win = 6,
    Draw = 3,
    Loss = 0,
};

char *getPath(int argc, char *argv[]) {
    if (argc == 1) {
        std::cout << "Expected input file as command-line argument." << std::endl;
        std::exit(1);
    }
    return argv[1];
}

const char PARSE_MOVE_OFFSET = 'A' - 1;
Move parseMove(char ch) {
    return static_cast<Move>(ch - PARSE_MOVE_OFFSET);
}

Result parseResult(char ch) {
    return static_cast<Result>((ch - 'X') * 3);
}

Move whatMoveDo(Move opMove, Result desiredResult) {
    if (desiredResult == Draw) {
        return opMove;
    }
    int off = desiredResult == Loss ? 1 : 0;
    return static_cast<Move>(((opMove + off) % 3) + 1);
}

// Does literally 0 data validation lmao.
int handleLine(std::string *line) {
    Move opMove = parseMove(line->at(0));
    Result expectedResult = parseResult(line->at(2));
    Move myMove = whatMoveDo(opMove, expectedResult);
    return expectedResult + myMove;
}

int solve(std::ifstream *file) {
    int acc = 0;
    std::string line;
    while (std::getline(*file, line)) {
        acc += handleLine(&line);
    }
    return acc;
}
