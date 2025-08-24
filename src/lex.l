use std::fs::File;
use std::io::Write;

#[derive(Clone, Debug)]
pub enum Token {
    Const,
    Id,
    Asign,
    Sum,
    Mul,
    Sub,
    Div,
    ParOpen,
    ParClose,
    CBOpen,
    CBClose,
    Semicolon
}


%%
%class Lexer
%field File lexer_out
%result_type Token


[0-9]+                          let _ = writeln!(self.lexer_out,"Const: {}", self.yytext());return Ok(Token::Const);
[a-zA-Z]([a-zA-Z]|[0-9])*       let _ = writeln!(self.lexer_out,"Id: {}", self.yytext());return Ok(Token::Id);
"="                             let _ = writeln!(self.lexer_out,"Asign: {}", self.yytext());return Ok(Token::Asign);
"+"                             let _ = writeln!(self.lexer_out,"Sum: {}", self.yytext());return Ok(Token::Sum);
"*"                             let _ = writeln!(self.lexer_out,"Mul: {}", self.yytext());return Ok(Token::Mul);
"-"                             let _ = writeln!(self.lexer_out,"Sub: {}", self.yytext());return Ok(Token::Sub);
"/"                             let _ = writeln!(self.lexer_out,"Div: {}", self.yytext());return Ok(Token::Div);
"("                             let _ = writeln!(self.lexer_out,"ParOpen: {}", self.yytext());return Ok(Token::ParOpen);
")"                             let _ = writeln!(self.lexer_out,"ParClose: {}", self.yytext());return Ok(Token::ParClose);
"{"                             let _ = writeln!(self.lexer_out,"CBOpen: {}", self.yytext());return Ok(Token::CBOpen);
"}"                             let _ = writeln!(self.lexer_out,"CBClose: {}", self.yytext());return Ok(Token::CBClose);
";"                             let _ = writeln!(self.lexer_out,"Semicolon: {}", self.yytext());return Ok(Token::Semicolon);
" "                             
(\r\n|\r|\n)
(\n\t)
(\t)

%%
