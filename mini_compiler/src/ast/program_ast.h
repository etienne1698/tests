#include <iostream>
#include <vector>

#include "./ast.h"
#include "./statements/statement_ast.h"

#ifndef PROGRAM_AST_H
#define PROGRAM_AST_H

class ProgramAST : public AST
{

  std::vector<std::unique_ptr<StatementAST>> statments;

public:
  ProgramAST(std::vector<std::unique_ptr<StatementAST>> statments);
};

#endif