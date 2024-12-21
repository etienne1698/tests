#include "./nasm_ast_visitor.h"
#include "nasm_ast_visitor.h"

using namespace std;

NasmASTVisitor::NasmASTVisitor() {}

void NasmASTVisitor::visit(NumberExprAST &expr) {
    cout << expr.val;
}

void NasmASTVisitor::visit(BinaryExprAST &expr) {
    // Implémentation de la méthode
}

void NasmASTVisitor::visit(VariableExprAST &expr) {
    // Implémentation de la méthode
}

void NasmASTVisitor::visit(CallExprAST &expr) {
    // Implémentation de la méthode
}

void NasmASTVisitor::visit(PrototypeStatementAST &stat) {
    // Implémentation de la méthode
}

void NasmASTVisitor::visit(ReturnStatementAST &stat) {
    // Implémentation de la méthode
}

void NasmASTVisitor::visit(PrintStatementAST &stat) {
    // Implémentation de la méthode
}