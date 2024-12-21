#include <string>

#include "../ast/expressions/expr_ast.h"
#include "../ast/expressions/numberexpr_ast.h"
#include "../ast/expressions/variableexpr_ast.h"
#include "../ast/expressions/callexpr_ast.h"
#include "../ast/expressions/binaryexpr_ast.h"

#include "../ast/statements/prototypestat_ast.h"
#include "../ast/statements/returnstat_ast.h"

#ifndef AST_VISITOR_H
#define AST_VISITOR_H

class ASTVisitor
{
public:
    virtual ~ASTVisitor();

    virtual void visit(NumberExprAST &expr) = 0;
    virtual void visit(VariableExprAST &expr) = 0;
    virtual void visit(CallExprAST &expr) = 0;
    virtual void visit(BinaryExprAST &expr) = 0;

    virtual void visit(PrototypeStatementAST &expr) = 0;
    virtual void visit(ReturnStatementAST &expr) = 0;
};

#endif