#include <iostream>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string.h>

#include "lexer/lexer.h"
#include "parser/parser.h"

using namespace std;

string getFileContent(string fileName)
{
    ifstream inFile;
    inFile.open(fileName);

    stringstream strStream;
    strStream << inFile.rdbuf();
    return strStream.str();
}

int main(int argc, char *argv[])
{

    if (argc < 2)
    {
        cout << "\033[31;1;4m" << "Filename required" << "\033[0m" << "\n";
        return 0;
    }

    Lexer lexer = Lexer(getFileContent(argv[1]));
    Parser parser = Parser(lexer);
    parser.parse();

    return 0;
}