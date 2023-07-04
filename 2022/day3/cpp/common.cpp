#include <iostream>
#include <vector>
#include <unordered_set>
#include "common.h"

char *getPath(int argc, char *argv[]) {
    if (argc == 1) {
        std::cout << "Expected input file as command-line argument." << std::endl;
        std::exit(1);
    }
    return argv[1];
}

int getPart(int argc, char *argv[]) {
    return argc != 2 && *argv[2] == '1' ? 1 : 2;
}

int getCharPriority(char ch) {
    if (ch > 'Z') { // forgot that uppercase chars come earlier for a min lmao 🤦‍♀️
        return ch - 'a' + 1;
    }
    else {
        return ch - 'A' + 27;
    }
}

char getCommonCharacter(std::vector<std::vector<char>> strings) {
    if (strings.size() == 0 || strings.at(0).size() == 0) {
        return '\0';
    }
    if (strings.size() == 1) {
        return strings.at(0).at(0);
    }

    std::unordered_set<char> prev, curr;

    // load in the first vector
    // probably a better way to do this? though also probably not much better than this lol
    for (int i = 0; i < strings[0].size(); i++) {
        prev.insert(strings[0][i]);
    }

    // starting at idx 1
    for (int i = 1; i < strings.size(); i++) {
        curr.clear();
        for (int j = 0; j < strings[i].size(); j++) {
            if (prev.find(strings[i][j]) != prev.end()) {
                curr.insert(strings[i][j]);
            }
        }
        if (curr.size() == 1) {
            return *curr.begin();
        }
        prev = curr;
    }

    return '\0';
}

