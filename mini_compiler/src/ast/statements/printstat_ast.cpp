#include "./printstat_ast.h"

PrintStatementAST::PrintStatementAST(std::unique_ptr<ExprAST> expr) : expr(std::move(expr)) {};