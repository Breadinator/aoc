aoc22d4.exe: main.o part1.o part2.o common.o
	g++ -o aoc22d4.exe main.o part1.o part2.o common.o

main.o: main.cpp part1.h part2.h
	g++ -c main.cpp

part1.o: part1.cpp part1.h common.h
	g++ -c part1.cpp

part2.o: part2.cpp part2.h common.h
	g++ -c part2.cpp

common.o: common.cpp common.h
	g++ -c common.cpp
