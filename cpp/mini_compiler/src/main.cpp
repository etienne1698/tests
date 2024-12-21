#include "./utils/utils.h"

#include "lexer/lexer.h"
#include "parser/parser.h"
#include "ast_visitor/nasm_ast_visitor.h"

using namespace std;

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