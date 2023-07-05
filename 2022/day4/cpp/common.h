#ifndef _AOC22D4_COMMON
#define _AOC22D4_COMMON

#include <stdbool.h>
#include <string>

struct Pairing {
    int a[2];
    int b[2];
};

extern char *getPath(int argc, char *argv[]);
extern int getPart(int argc, char *argv[]);

extern struct Pairing parse(std::string line);
extern bool isSubset(struct Pairing pairing);
extern bool hasOverlap(struct Pairing pairing);

#endif
