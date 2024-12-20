#include <string>
#include <cctype>

#include "lexer.h"

Lexer::Lexer(const std::string &input) : source(input) {}

char Lexer::getNextChar()
{
    if (position < source.size())
    {
        return source[position++];
    }
    return '\0';
}

char Lexer::getNextToken()
{
    char lastChar = getNextChar();

    while (isspace(lastChar))
    {
        lastChar = getNextChar();
    }

    if (lastChar == '\0')
    {
        return TOKEN_EOF;
    }

    if (isalpha(lastChar))
    {
        identifierStr = lastChar;
        while (isalnum(lastChar = getNextChar()))
        {
            identifierStr += lastChar;
        }
        if (identifierStr == "def")
        {
            return TOKEN_DEF;
        }
        return TOKEN_IDENTIFIER;
    }

    if (isdigit(lastChar) || lastChar == '.')
    { // Number: [0-9.]+
        std::string NumStr;
        do
        {
            NumStr += lastChar;
            lastChar = getNextChar();
        } while (isdigit(lastChar) || lastChar == '.');

        numVal = strtod(NumStr.c_str(), nullptr);
        return TOKEN_NUMBER;
    }

    return lastChar;
}

std::string Lexer::getIdentifier()
{
    return identifierStr;
}

double Lexer::getNumVal()
{
    return numVal;
}