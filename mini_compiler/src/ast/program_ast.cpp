#include "program_ast.h"

ProgramAST::ProgramAST(std::vector<std::unique_ptr<StatementAST>> statments) : statments(std::move(statments)) {}