#ifndef _AOC22D3_COMMON
#define _AOC22D3_COMMON

#include <vector>

// Returns the path in argv[1].
// Exits early if none provided.
extern char *getPath(int argc, char *argv[]);

// Returns 1 if part 1 is specified as the second CLArg, othewise 2.
// Assumes that argc > 1 (i.e. the path has already been checked for and exited early if none given).
extern int getPart(int argc, char *argv[]);

extern int getCharPriority(char ch);

// Returns '\0' (null) if there's no strings or if there's no common characters.
// If there's multiple common characters, it just picks the first that happens to be in the set.
extern char getCommonCharacter(std::vector<std::vector<char>> strings);

#endif
