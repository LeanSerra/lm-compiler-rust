use crate::grammar::TokenKind;

%%
%class Lexer
%result_type TokenKind


"int"                                                           return Ok(TokenKind::TokenInt);
"float"                                                         return Ok(TokenKind::TokenFloat);
"string"                                                        return Ok(TokenKind::TokenString);
"init"                                                          return Ok(TokenKind::TokenInit);
("-"?[0-9]+)                                                    return Ok(TokenKind::TokenIntLiteral);
("-"?([0-9]+("."[0-9]*)?|"."[0-9]+)([eE][-+]?[0-9]+)?)          return Ok(TokenKind::TokenFloatLiteral);
[a-zA-Z]([a-zA-Z]|[0-9])*                                       return Ok(TokenKind::TokenId);
\"[^\n"]*\"                                                     return Ok(TokenKind::TokenStringLiteral)
":""="                                                          return Ok(TokenKind::TokenAssign);
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
" "                             
(\r\n|\r|\n)
(\n\t)
(\t)

%%
