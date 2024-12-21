#include "./nasm_ast_visitor.h"
#include "nasm_ast_visitor.h"

using namespace std;

void NasmASTVisitor::visit(NumberExprAST &expr) {
    cout << expr.val;
}