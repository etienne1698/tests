#include <string>

#include "./ast_visitor.h"

#ifndef NASMAST_VISITOR_H
#define NASMAST_VISITOR_H

class NasmASTVisitor : ASTVisitor
{
private:
    std::string result = "";

    void visit(NumberExprAST &expr) override;
};

#endif