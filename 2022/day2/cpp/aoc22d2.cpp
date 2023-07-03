#include <fstream>
#include <iostream>
#include <string>
#include "lib.h"

int main(int argc, char *argv[]) {
    char *path = getPath(argc, argv);
    std::ifstream file(path);

    if (!file.is_open()) {
        std::cout << "Couldn't open file at " << path << std::endl;
        return 1;
    }

    int answer = solve(&file);
    std::cout << std::to_string(answer) << std::endl;

    file.close();
    return 0;
}
