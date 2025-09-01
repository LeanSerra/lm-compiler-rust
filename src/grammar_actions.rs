/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use super::grammar::{Context, TokenKind};
use super::grammar_lexer::Input;
use crate::OUTPUT_FILE;
use rustemo::Token as RustemoToken;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type TokenInt = String;
pub fn token_int(_ctx: &Ctx, token: Token) -> TokenInt {
    write_lexer_output(&token, "INT");
    token.value.into()
}
pub type TokenFloat = String;
pub fn token_float(_ctx: &Ctx, token: Token) -> TokenFloat {
    write_lexer_output(&token, "FLOAT");
    token.value.into()
}
pub type TokenString = String;
pub fn token_string(_ctx: &Ctx, token: Token) -> TokenString {
    write_lexer_output(&token, "STRING");
    token.value.into()
}
pub type TokenIntLiteral = String;
pub fn token_int_literal(_ctx: &Ctx, token: Token) -> TokenIntLiteral {
    write_lexer_output(&token, "INTLITERAL");
    token.value.into()
}
pub type TokenFloatLiteral = String;
pub fn token_float_literal(_ctx: &Ctx, token: Token) -> TokenFloatLiteral {
    write_lexer_output(&token, "FLOATLITERAL");
    token.value.into()
}
pub type TokenStringLiteral = String;
pub fn token_string_literal(_ctx: &Ctx, token: Token) -> TokenStringLiteral {
    write_lexer_output(&token, "STRINGLITERAL");
    token.value.into()
}
pub type TokenId = String;
pub fn token_id(_ctx: &Ctx, token: Token) -> TokenId {
    write_lexer_output(&token, "ID");
    token.value.into()
}
pub type TokenAssign = String;
pub fn token_assign(_ctx: &Ctx, token: Token) -> TokenAssign {
    write_lexer_output(&token, "ASSIGN");
    token.value.into()
}
pub type TokenParOpen = String;
pub fn token_par_open(_ctx: &Ctx, token: Token) -> TokenParOpen {
    write_lexer_output(&token, "PAROPEN");
    token.value.into()
}
pub type TokenParClose = String;
pub fn token_par_close(_ctx: &Ctx, token: Token) -> TokenParClose {
    write_lexer_output(&token, "PARCLOSE");
    token.value.into()
}
pub type TokenCBOpen = String;
pub fn token_cbopen(_ctx: &Ctx, token: Token) -> TokenCBOpen {
    write_lexer_output(&token, "CBOPEN");
    token.value.into()
}
pub type TokenCBClose = String;
pub fn token_cbclose(_ctx: &Ctx, token: Token) -> TokenCBClose {
    write_lexer_output(&token, "CBCLOSE");
    token.value.into()
}
pub type TokenColon = String;
pub fn token_colon(_ctx: &Ctx, token: Token) -> TokenColon {
    write_lexer_output(&token, "COLON");
    token.value.into()
}
pub type TokenInit = String;
pub fn token_init(_ctx: &Ctx, token: Token) -> TokenInit {
    write_lexer_output(&token, "INIT");
    token.value.into()
}
pub type TokenComma = String;
pub fn token_comma(_ctx: &Ctx, token: Token) -> TokenComma {
    write_lexer_output(&token, "COMMA");
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct Program {
    pub token_id: TokenId,
    pub token_par_open: TokenParOpen,
    pub token_par_close: TokenParClose,
    pub token_cbopen: TokenCBOpen,
    pub body: Body,
    pub token_cbclose: TokenCBClose,
}
pub fn program_program(
    _ctx: &Ctx,
    token_id: TokenId,
    token_par_open: TokenParOpen,
    token_par_close: TokenParClose,
    token_cbopen: TokenCBOpen,
    body: Body,
    token_cbclose: TokenCBClose,
) -> Program {
    write_parser_output(
        &format!(
            "<Program> -> {token_id} {token_par_open} {token_par_close} {token_cbopen} <Body> {token_cbclose}"
        ),
    );
    Program {
        token_id,
        token_par_open,
        token_par_close,
        token_cbopen,
        body,
        token_cbclose,
    }
}
#[derive(Debug, Clone)]
pub struct Body {
    pub token_init: TokenInit,
    pub init_body: InitBody,
    pub expressions: Expressions,
}
pub fn body_body(
    _ctx: &Ctx,
    token_init: TokenInit,
    init_body: InitBody,
    expressions: Expressions,
) -> Body {
    write_parser_output(&format!("<Body> -> {token_init} <InitBody> <Expressions>"));
    Body {
        token_init,
        init_body,
        expressions,
    }
}
#[derive(Debug, Clone)]
pub struct InitBody {
    pub token_cbopen: TokenCBOpen,
    pub var_declarations: VarDeclarations,
    pub token_cbclose: TokenCBClose,
}
pub fn init_body_init_body(
    _ctx: &Ctx,
    token_cbopen: TokenCBOpen,
    var_declarations: VarDeclarations,
    token_cbclose: TokenCBClose,
) -> InitBody {
    write_parser_output(
        &format!("<InitBody> -> {token_cbopen} <VarDeclarations> {token_cbclose}"),
    );
    InitBody {
        token_cbopen,
        var_declarations,
        token_cbclose,
    }
}
#[derive(Debug, Clone)]
pub struct VarDeclarationsRecursive {
    pub var_declaration: VarDeclaration,
    pub var_declarations: Box<VarDeclarations>,
}
#[derive(Debug, Clone)]
pub enum VarDeclarations {
    VarDeclarationsSingle(VarDeclaration),
    VarDeclarationsRecursive(VarDeclarationsRecursive),
}
pub fn var_declarations_var_declarations_single(
    _ctx: &Ctx,
    var_declaration: VarDeclaration,
) -> VarDeclarations {
    write_parser_output("<VarDeclarations> -> <VarDeclaration>");
    VarDeclarations::VarDeclarationsSingle(var_declaration)
}
pub fn var_declarations_var_declarations_recursive(
    _ctx: &Ctx,
    var_declaration: VarDeclaration,
    var_declarations: VarDeclarations,
) -> VarDeclarations {
    write_parser_output("<VarDeclarations> -> <VarDeclaration> <VarDeclaration>");
    VarDeclarations::VarDeclarationsRecursive(VarDeclarationsRecursive {
        var_declaration,
        var_declarations: Box::new(var_declarations),
    })
}
#[derive(Debug, Clone)]
pub struct VarDeclarationSingle {
    pub token_id: TokenId,
    pub token_colon: TokenColon,
    pub data_type: Data_Type,
}
#[derive(Debug, Clone)]
pub struct VarDeclarationRecursive {
    pub token_id: TokenId,
    pub token_comma: TokenComma,
    pub var_declaration: Box<VarDeclaration>,
}
#[derive(Debug, Clone)]
pub enum VarDeclaration {
    VarDeclarationSingle(VarDeclarationSingle),
    VarDeclarationRecursive(VarDeclarationRecursive),
}
pub fn var_declaration_var_declaration_single(
    _ctx: &Ctx,
    token_id: TokenId,
    token_colon: TokenColon,
    data_type: Data_Type,
) -> VarDeclaration {
    write_parser_output(
        &format!("<VarDeclaration> -> {token_id} {token_colon} <Data_Type>"),
    );
    VarDeclaration::VarDeclarationSingle(VarDeclarationSingle {
        token_id,
        token_colon,
        data_type,
    })
}
pub fn var_declaration_var_declaration_recursive(
    _ctx: &Ctx,
    token_id: TokenId,
    token_comma: TokenComma,
    var_declaration: VarDeclaration,
) -> VarDeclaration {
    write_parser_output(
        &format!("<VarDeclaration> -> {token_id} {token_comma} <VarDeclaration>"),
    );
    VarDeclaration::VarDeclarationRecursive(VarDeclarationRecursive {
        token_id,
        token_comma,
        var_declaration: Box::new(var_declaration),
    })
}
#[derive(Debug, Clone)]
pub struct ExpressionRecursive {
    pub expression: Expression,
    pub expressions: Box<Expressions>,
}
#[derive(Debug, Clone)]
pub enum Expressions {
    ExpressionSingle(Expression),
    ExpressionRecursive(ExpressionRecursive),
}
pub fn expressions_expression_single(_ctx: &Ctx, expression: Expression) -> Expressions {
    write_parser_output("<Expression> -> <Expression>");
    Expressions::ExpressionSingle(expression)
}
pub fn expressions_expression_recursive(
    _ctx: &Ctx,
    expression: Expression,
    expressions: Expressions,
) -> Expressions {
    write_parser_output("<Expressions> -> <Expression> <Expressions>");
    Expressions::ExpressionRecursive(ExpressionRecursive {
        expression,
        expressions: Box::new(expressions),
    })
}
pub type Expression = Assignment;
pub fn expression_expression_assignment(
    _ctx: &Ctx,
    assignment: Assignment,
) -> Expression {
    write_parser_output("<Expression> -> <Assignment>");
    assignment
}
#[derive(Debug, Clone)]
pub struct Assignment {
    pub token_id: TokenId,
    pub token_assign: TokenAssign,
    pub literal: Literal,
}
pub fn assignment_assignment(
    _ctx: &Ctx,
    token_id: TokenId,
    token_assign: TokenAssign,
    literal: Literal,
) -> Assignment {
    write_parser_output(&format!("<Assignment> -> {token_id} {token_assign} <Literal>"));
    Assignment {
        token_id,
        token_assign,
        literal,
    }
}
#[derive(Debug, Clone)]
pub enum Literal {
    IntegerLiteral(TokenIntLiteral),
    FloatLiteral(TokenFloatLiteral),
    StringLiteral(TokenStringLiteral),
}
pub fn literal_integer_literal(
    _ctx: &Ctx,
    token_int_literal: TokenIntLiteral,
) -> Literal {
    write_parser_output(&format!("<Literal> -> {token_int_literal}"));
    Literal::IntegerLiteral(token_int_literal)
}
pub fn literal_float_literal(
    _ctx: &Ctx,
    token_float_literal: TokenFloatLiteral,
) -> Literal {
    write_parser_output(&format!("<Literal> -> {token_float_literal}"));
    Literal::FloatLiteral(token_float_literal)
}
pub fn literal_string_literal(
    _ctx: &Ctx,
    token_string_literal: TokenStringLiteral,
) -> Literal {
    write_parser_output(&format!("<Literal> -> {token_string_literal}"));
    Literal::StringLiteral(token_string_literal)
}
#[derive(Debug, Clone)]
pub enum Data_Type {
    IntType(TokenInt),
    FloatType(TokenFloat),
    StringType(TokenString),
}
pub fn data_type_int_type(_ctx: &Ctx, token_int: TokenInt) -> Data_Type {
    write_parser_output(&format!("<Data_Type> -> {token_int}"));
    Data_Type::IntType(token_int)
}
pub fn data_type_float_type(_ctx: &Ctx, token_float: TokenFloat) -> Data_Type {
    write_parser_output(&format!("<Data_Type> -> {token_float}"));
    Data_Type::FloatType(token_float)
}
pub fn data_type_string_type(_ctx: &Ctx, token_string: TokenString) -> Data_Type {
    write_parser_output(&format!("<Data_Type> -> {token_string}"));
    Data_Type::StringType(token_string)
}
pub fn open_lexer_output_file() -> Result<File, std::io::Error> {
    let mut path = PathBuf::from(OUTPUT_FILE.get().unwrap());
    path.set_extension("lexer");
    std::fs::OpenOptions::new().append(true).open(path)
}
pub fn write_lexer_output(token: &Token, tokenName: &str) -> Result<(), std::io::Error> {
    let mut file = open_lexer_output_file()?;
    writeln!(file, "{tokenName}: {}", token.value)
}
pub fn open_parser_output_file() -> Result<File, std::io::Error> {
    let mut path = PathBuf::from(OUTPUT_FILE.get().unwrap());
    path.set_extension("parser");
    std::fs::OpenOptions::new().append(true).open(path)
}
pub fn write_parser_output(rule: &str) -> Result<(), std::io::Error> {
    let mut file = open_parser_output_file()?;
    writeln!(file, "{rule}")
}
