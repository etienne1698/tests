#include "map"
#include <cctype>
#include <string>
#include "memory"

#include "../utils/utils.h"

#include "../lexer/lexer.h"

#include "../ast/expressions/expr_ast.h"
#include "../ast/expressions/numberexpr_ast.h"
#include "../ast/expressions/variableexpr_ast.h"
#include "../ast/expressions/callexpr_ast.h"
#include "../ast/expressions/binaryexpr_ast.h"

#include "../ast/statements/prototypestat_ast.h"
#include "../ast/statements/returnstat_ast.h"
#include "../ast/statements/printstat_ast.h"
#include "../ast/program_ast.h"

#ifndef PARSER_H
#define PARSER_H

class Parser
{
    Lexer lexer;
    std::unique_ptr<StatementAST> parseFuncDefStat();
    std::unique_ptr<StatementAST> parsePrintStat();
    

    std::unique_ptr<ExprAST> parseExpr();

    std::unique_ptr<ExprAST> parsePrimaryExpr();
    std::unique_ptr<ExprAST> parseParentesisExpr();
    std::unique_ptr<ExprAST> parseIdentifierExpr();
    std::unique_ptr<ExprAST> parseNumberExpr();
    std::unique_ptr<ExprAST> parseBinOp(int exprPrec,
                                           std::unique_ptr<ExprAST> left);

    char currentToken;
    void getNextToken();
    int getTokenPrecedence();
    static std::map<char, int> BinopPrecedence;

public:
    explicit Parser(const Lexer &lexer);
    std::unique_ptr<ProgramAST> parse();
};

#endif