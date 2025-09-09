use crate::grammar::TokenKind;

%%
%class Lexer
%result_type TokenKind


#\+(.|\n)*\+#
"int"                                                           return Ok(TokenKind::TokenInt);
"float"                                                         return Ok(TokenKind::TokenFloat);
"string"                                                        return Ok(TokenKind::TokenString);
"init"                                                          return Ok(TokenKind::TokenInit);
"while"                                                         return Ok(TokenKind::TokenWhile);
"true"                                                          return Ok(TokenKind::TokenTrue);
"false"                                                         return Ok(TokenKind::TokenFalse);
"if"                                                            return Ok(TokenKind::TokenIf);
"else"                                                          return Ok(TokenKind::TokenElse);
"and"                                                           return Ok(TokenKind::TokenAnd);
"or"                                                            return Ok(TokenKind::TokenOr);
"not"                                                           return Ok(TokenKind::TokenNot);
"read"                                                          return Ok(TokenKind::TokenRead);
"write"                                                         return Ok(TokenKind::TokenWrite);
"isZero"                                                        return Ok(TokenKind::TokenIsZero);
"convDate"                                                      return Ok(TokenKind::TokenConvDate);
("-"?[0-9]+)                                                    return Ok(TokenKind::TokenIntLiteral);
("-"?([0-9]+("."[0-9]*)?|"."[0-9]+)([eE][-+]?[0-9]+)?)          return Ok(TokenKind::TokenFloatLiteral);
[a-zA-Z]([a-zA-Z]|[0-9])*                                       return Ok(TokenKind::TokenId);
\"([^\"\\\r\n]|\\.)*\"                                          return Ok(TokenKind::TokenStringLiteral);
":="                                                            return Ok(TokenKind::TokenAssign);
"+"                                                             return Ok(TokenKind::TokenSum);
"*"                                                             return Ok(TokenKind::TokenMul);
"-"                                                             return Ok(TokenKind::TokenSub);
"/"                                                             return Ok(TokenKind::TokenDiv);
"("                                                             return Ok(TokenKind::TokenParOpen);
")"                                                             return Ok(TokenKind::TokenParClose);
"{"                                                             return Ok(TokenKind::TokenCBOpen);
"}"                                                             return Ok(TokenKind::TokenCBClose);
";"                                                             return Ok(TokenKind::TokenSemicolon);
","                                                             return Ok(TokenKind::TokenComma);
":"                                                             return Ok(TokenKind::TokenColon);
"=="                                                            return Ok(TokenKind::TokenEqual);
"!="                                                            return Ok(TokenKind::TokenNotEqual);
"<"                                                             return Ok(TokenKind::TokenLess);
"<="                                                            return Ok(TokenKind::TokenLessEqual);
">"                                                             return Ok(TokenKind::TokenGreater);
">="                                                            return Ok(TokenKind::TokenGreaterEqual);
" "
(\r\n|\r|\n)
(\n\t)
(\t)

%%
