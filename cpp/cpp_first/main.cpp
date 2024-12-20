#include <iostream>
#include <string.h>
using namespace std;

int main(int argc, char *argv[])
{
    int a;

    cout << (_stricmp(argv[1], "test") == 0 ? "OK" : "NOP") << "\n"
         << argv[1];

    return 0;
}