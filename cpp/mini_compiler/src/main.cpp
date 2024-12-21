#include <iostream>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string.h>

#include "./utils/utils.h"

#include "lexer/lexer.h"
#include "parser/parser.h"
#include "ast_visitor/nasm_ast_visitor.h"

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
        logError("Filename required");
        return 0;
    }

    Lexer lexer = Lexer(getFileContent(argv[1]));
    Parser parser = Parser(lexer);
    auto ast = parser.parse();

    auto nasmASTVisitor = NasmASTVisitor();

    return 0;
}