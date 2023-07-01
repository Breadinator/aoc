#include <iostream>
#include <fstream>
#include <string>
using namespace std;

char *getPath(int argc, char *argv[]) {
    if (argc == 1) {
        cout << "Expected input file path as command-line argument." << endl;
        exit(1);
    }
    return argv[1];
}

struct Top3 {
    int a;
    int b;
    int c;
};

void pushTop3(Top3 *top3, int newValue) {
    if (top3->a < newValue) {
        top3->c = top3->b;
        top3->b = top3->a;
        top3->a = newValue;
    }
    else if (top3->b < newValue) {
        top3->c = top3->b;
        top3->b = newValue;
    }
    else if (top3->c < newValue) {
        top3->c = newValue;
    }
}

int sumTop3(Top3 *top3) {
    return top3->a + top3->b + top3->c;
}

int main(int argc, char *argv[]) {
    char *path = getPath(argc, argv);
    ifstream file(path);

    if (!file.is_open()) {
        cout << "Couldn't open file at " << path << endl;
        return 1;
    }

    Top3 top3 = Top3{};
    string line;
    int acc = 0;

    while (getline(file, line)) {
        if (line.empty()) {
            pushTop3(&top3, acc);
            acc = 0;
        }
        else {
            acc += stoi(line);
        }
    }
    // leftover value
    pushTop3(&top3, acc);

    cout << "Top 3:" << endl << to_string(top3.a) << endl << to_string(top3.b) << endl << to_string(top3.c) << endl << endl
        << "Sum of top 3: " << to_string(sumTop3(&top3));

    file.close();
    return 0;
}
