#include "./utils.h"

using namespace std;

void logError(string err) {
    cout << "\033[31;1;4m" << err << "\033[0m" << "\n";
}