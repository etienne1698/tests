#include <string>

#include "./ast_visitor.h"

#ifndef NASMAST_VISITOR_H
#define NASMAST_VISITOR_H

class NasmASTVisitor : ASTVisitor
{
private:
    std::string result = "";

public:
    NasmASTVisitor();
    void visit(NumberExprAST &expr) override;
    void visit(BinaryExprAST &expr) override;
    void visit(VariableExprAST &expr) override;
    void visit(CallExprAST &expr) override;

    void visit(PrototypeStatementAST &expr) override;
    void visit(ReturnStatementAST &expr) override;
    void visit(PrintStatementAST &expr) override;

};

#endif