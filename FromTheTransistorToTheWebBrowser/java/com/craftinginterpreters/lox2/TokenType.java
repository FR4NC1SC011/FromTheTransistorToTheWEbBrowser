package com.craftinginterpreters.lox2;

enum TokenType {
  // single character tokens
  LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  // one or two character tokens
  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  // literals
  IDENTIFIER, STRING, NUMBER,

  // keywords
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR, 
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE, BREAK,


  EOF
}
