use crate::grammar::TokenKind;
use crate::grammar_lexer::log_error;
use crate::read_source_to_string;

%%
%class Lexer
%field usize offset
%result_type TokenKind


#\+([^\"\\\r\n#]|\\.)*\+#
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
([0-9][0-9]-[0-9][0-9]-[0-9][0-9][0-9][0-9])                    return Ok(TokenKind::TokenDate);
([0-9]+)                                                        {
                                                                    if let Err(e) = self.yytext().parse::<u64>() {
                                                                        log_error(
                                                                            self.yytextpos(),
                                                                            crate::CompilerError::Lexer(format!("Invalid integer literal {e}")),
                                                                            self.offset,
                                                                            &read_source_to_string().unwrap(),
                                                                        );
                                                                        std::process::exit(1)
                                                                    }
                                                                    return Ok(TokenKind::TokenIntLiteral);
                                                                }
(([0-9]+("."[0-9]*)?|"."[0-9]+)([eE][-+]?[0-9]+)?)              {
                                                                    match self.yytext().parse::<f32>() {
                                                                        Err(e) => {
                                                                            log_error(
                                                                                self.yytextpos(),
                                                                                crate::CompilerError::Lexer(format!("Invalid float literal {e}")),
                                                                                self.offset,
                                                                                &read_source_to_string().unwrap(),
                                                                            );
                                                                            std::process::exit(1)
                                                                        }
                                                                        Ok(value) => {
                                                                            if !value.is_normal() {
                                                                                log_error(
                                                                                self.yytextpos(),
                                                                                crate::CompilerError::Lexer(format!("Invalid float literal")),
                                                                                self.offset,
                                                                                &read_source_to_string().unwrap(),
                                                                            );
                                                                            std::process::exit(1)
                                                                            }
                                                                        }
                                                                    };
                                                                    return Ok(TokenKind::TokenFloatLiteral);
                                                                }
[a-zA-Z]([a-zA-Z]|[0-9])*                                       return Ok(TokenKind::TokenId);
\"([^\"\\\r\n#]|\\.)*\"                                         {
                                                                    if self.yytext().len() > 256 {
                                                                        log_error(
                                                                            self.yytextpos(),
                                                                            crate::CompilerError::Lexer(format!("Invalid string length {}", self.yytext().len())),
                                                                            self.offset,
                                                                            &read_source_to_string().unwrap(),
                                                                        );
                                                                        std::process::exit(1);
                                                                    }
                                                                    return Ok(TokenKind::TokenStringLiteral);
                                                                }
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
