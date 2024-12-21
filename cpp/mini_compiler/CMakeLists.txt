cmake_minimum_required(VERSION 3.10)

project(MiniCompiler)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(SRCS
    src/main.cpp src/lexer/lexer.cpp 
    src/parser/parser.cpp 
    src/ast/expressions/expr_ast.cpp 
    src/ast/expressions/binaryexpr_ast.cpp 
    src/ast/expressions/callexpr_ast.cpp 
    src/ast/expressions/numberexpr_ast.cpp 
    src/ast/expressions/variableexpr_ast.cpp 
    src/ast/statements/statement_ast.cpp 
    src/ast/statements/prototypestat_ast.cpp 
    src/ast/statements/returnstat_ast.cpp
    src/ast/statements/printstat_ast.cpp

    src/ast_visitor/ast_visitor.cpp
    src/ast_visitor/nasm_ast_visitor.cpp

    src/ast/ast.cpp
    src/ast/program_ast.cpp
    )

add_executable(compile ${SRCS})