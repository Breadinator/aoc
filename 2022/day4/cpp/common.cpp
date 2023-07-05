#include <stdbool.h>
#include <string>
#include "common.h"

char *getPath(int argc, char *argv[]) {
    if (argc < 3) {
        return nullptr;
    } else {
        return argv[2];
    }
}

int getPart(int argc, char *argv[]) {
    return argc > 1 && *argv[1] == '1' ? 1 : 2;
}

struct Pairing parse(std::string line) {
    auto split = line.find(',');

    // parse a
    std::string curr = line.substr(0, split);
    auto subSplit = curr.find('-');
    int aMin = std::stoi(curr.substr(0, subSplit));
    int aMax = std::stoi(curr.substr(subSplit + 1, curr.size() - subSplit - 1));

    // parse b
    curr = line.substr(split + 1, line.size() - split - 1);
    subSplit = curr.find('-');
    int bMin = std::stoi(curr.substr(0, subSplit));
    int bMax = std::stoi(curr.substr(subSplit + 1, curr.size() - subSplit - 1));

    return Pairing {{aMin, aMax}, {bMin, bMax}};
}

bool isSubset(struct Pairing pairing) {
    return (pairing.a[0] >= pairing.b[0] && pairing.a[1] <= pairing.b[1])
        || (pairing.b[0] >= pairing.a[0] && pairing.b[1] <= pairing.a[1]);
}

bool hasOverlap(struct Pairing pairing) {
    return (pairing.a[0] <= pairing.b[0] && pairing.a[1] >= pairing.b[0])
            || (pairing.b[0] <= pairing.a[0] && pairing.b[1] >= pairing.a[0])
            || (pairing.a[1] >= pairing.b[1] && pairing.a[0] <= pairing.b[1])
            || (pairing.b[1] >= pairing.a[1] && pairing.b[0] <= pairing.a[1]);
}

