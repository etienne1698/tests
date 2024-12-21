#include <iostream>
#include <vector>
#include "./statement_ast.h"
#include "../expressions/expr_ast.h"

#ifndef PRINTSTAT_AST_H
#define PRINTSTAT_AST_H

class PrintStatementAST : public StatementAST
{
    std::unique_ptr<ExprAST> expr;

public:
    PrintStatementAST(std::unique_ptr<ExprAST> expr);
};

#endif