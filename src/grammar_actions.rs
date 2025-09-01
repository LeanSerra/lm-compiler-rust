/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use super::grammar::{Context, TokenKind};
use super::grammar_lexer::Input;
use crate::context::{
    write_to_lexer_file, write_to_parser_file, write_to_symbol_table_file,
};
use rustemo::Token as RustemoToken;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type TokenInt = String;
pub fn token_int(_ctx: &Ctx, token: Token) -> TokenInt {
    write_to_lexer_file(&format!("INT: {}", token.value));
    token.value.into()
}
pub type TokenFloat = String;
pub fn token_float(_ctx: &Ctx, token: Token) -> TokenFloat {
    write_to_lexer_file(&format!("FLOAT: {}", token.value));
    token.value.into()
}
pub type TokenString = String;
pub fn token_string(_ctx: &Ctx, token: Token) -> TokenString {
    write_to_lexer_file(&format!("STRING: {}", token.value));
    token.value.into()
}
pub type TokenIntLiteral = String;
pub fn token_int_literal(_ctx: &Ctx, token: Token) -> TokenIntLiteral {
    write_to_lexer_file(&format!("INT_LITERAL: {}", token.value));
    token.value.into()
}
pub type TokenFloatLiteral = String;
pub fn token_float_literal(_ctx: &Ctx, token: Token) -> TokenFloatLiteral {
    write_to_lexer_file(&format!("FLOAT_LITERAL: {}", token.value));
    token.value.into()
}
pub type TokenStringLiteral = String;
pub fn token_string_literal(_ctx: &Ctx, token: Token) -> TokenStringLiteral {
    write_to_lexer_file(&format!("STRING_LITERAL: {}", token.value));
    token.value.into()
}
pub type TokenId = String;
pub fn token_id(_ctx: &Ctx, token: Token) -> TokenId {
    write_to_lexer_file(&format!("ID: {}", token.value));
    token.value.into()
}
pub type TokenAssign = String;
pub fn token_assign(_ctx: &Ctx, token: Token) -> TokenAssign {
    write_to_lexer_file(&format!("ASSIGN: {}", token.value));
    token.value.into()
}
pub type TokenParOpen = String;
pub fn token_par_open(_ctx: &Ctx, token: Token) -> TokenParOpen {
    write_to_lexer_file(&format!("PAROPEN: {}", token.value));
    token.value.into()
}
pub type TokenParClose = String;
pub fn token_par_close(_ctx: &Ctx, token: Token) -> TokenParClose {
    write_to_lexer_file(&format!("PARCLOSE: {}", token.value));
    token.value.into()
}
pub type TokenCBOpen = String;
pub fn token_cbopen(_ctx: &Ctx, token: Token) -> TokenCBOpen {
    write_to_lexer_file(&format!("CBOPEN: {}", token.value));
    token.value.into()
}
pub type TokenCBClose = String;
pub fn token_cbclose(_ctx: &Ctx, token: Token) -> TokenCBClose {
    write_to_lexer_file(&format!("CBCLOSE: {}", token.value));
    token.value.into()
}
pub type TokenColon = String;
pub fn token_colon(_ctx: &Ctx, token: Token) -> TokenColon {
    write_to_lexer_file(&format!("COLON: {}", token.value));
    token.value.into()
}
pub type TokenInit = String;
pub fn token_init(_ctx: &Ctx, token: Token) -> TokenInit {
    write_to_lexer_file(&format!("INIT: {}", token.value));
    token.value.into()
}
pub type TokenComma = String;
pub fn token_comma(_ctx: &Ctx, token: Token) -> TokenComma {
    write_to_lexer_file(&format!("COMMA: {}", token.value));
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
    write_to_parser_file(
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
    write_to_parser_file(&format!("<Body> -> {token_init} <InitBody> <Expressions>"));
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
    write_to_parser_file(
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
    var_declaration.write_to_symbol_table();
    write_to_parser_file("<VarDeclarations> -> <VarDeclaration>");
    VarDeclarations::VarDeclarationsSingle(var_declaration)
}
pub fn var_declarations_var_declarations_recursive(
    _ctx: &Ctx,
    var_declaration: VarDeclaration,
    var_declarations: VarDeclarations,
) -> VarDeclarations {
    var_declaration.write_to_symbol_table();
    write_to_parser_file("<VarDeclarations> -> <VarDeclaration> <VarDeclaration>");
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
    write_to_parser_file(
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
    write_to_parser_file(
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
    write_to_parser_file("<Expression> -> <Expression>");
    Expressions::ExpressionSingle(expression)
}
pub fn expressions_expression_recursive(
    _ctx: &Ctx,
    expression: Expression,
    expressions: Expressions,
) -> Expressions {
    write_to_parser_file("<Expressions> -> <Expression> <Expressions>");
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
    write_to_parser_file("<Expression> -> <Assignment>");
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
    write_to_parser_file(
        &format!("<Assignment> -> {token_id} {token_assign} <Literal>"),
    );
    /// The rhs of an assignment to a literal is a symbol
    literal.write_to_symbol_table();
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
    write_to_parser_file(&format!("<Literal> -> {token_int_literal}"));
    Literal::IntegerLiteral(token_int_literal)
}
pub fn literal_float_literal(
    _ctx: &Ctx,
    token_float_literal: TokenFloatLiteral,
) -> Literal {
    write_to_parser_file(&format!("<Literal> -> {token_float_literal}"));
    Literal::FloatLiteral(token_float_literal)
}
pub fn literal_string_literal(
    _ctx: &Ctx,
    token_string_literal: TokenStringLiteral,
) -> Literal {
    write_to_parser_file(&format!("<Literal> -> {token_string_literal}"));
    /// We remove the "" from the string literal
    Literal::StringLiteral(token_string_literal.replace("\"", ""))
}
#[derive(Debug, Clone)]
pub enum Data_Type {
    IntType(TokenInt),
    FloatType(TokenFloat),
    StringType(TokenString),
}
pub fn data_type_int_type(_ctx: &Ctx, token_int: TokenInt) -> Data_Type {
    write_to_parser_file(&format!("<Data_Type> -> {token_int}"));
    Data_Type::IntType(token_int)
}
pub fn data_type_float_type(_ctx: &Ctx, token_float: TokenFloat) -> Data_Type {
    write_to_parser_file(&format!("<Data_Type> -> {token_float}"));
    Data_Type::FloatType(token_float)
}
pub fn data_type_string_type(_ctx: &Ctx, token_string: TokenString) -> Data_Type {
    write_to_parser_file(&format!("<Data_Type> -> {token_string}"));
    Data_Type::StringType(token_string)
}
impl Literal {
    pub fn write_to_symbol_table(&self) {
        match self {
            Literal::IntegerLiteral(int) => {
                write_to_symbol_table_file(
                    &format!("{}|{}|{}|{}", int, "CONST_INT", int, int.len()),
                )
            }
            Literal::FloatLiteral(float) => {
                write_to_symbol_table_file(
                    &format!("{}|{}|{}|{}", float, "CONST_FLOAT", float, float.len()),
                )
            }
            Literal::StringLiteral(string) => {
                write_to_symbol_table_file(
                    &format!("{}|{}|{}|{}", string, "CONST_STRING", string, string.len()),
                )
            }
        };
    }
}
impl VarDeclaration {
    pub fn write_to_symbol_table(&self) -> Data_Type {
        match self {
            Self::VarDeclarationSingle(single) => {
                write_to_symbol_table_file(
                    &format!(
                        "{}|{}|{}|{}", single.token_id, single.data_type.to_string(),
                        "-", single.token_id.len()
                    ),
                );
                single.data_type.clone()
            }
            Self::VarDeclarationRecursive(recursive) => {
                let data_type = recursive.var_declaration.write_to_symbol_table();
                write_to_symbol_table_file(
                    &format!(
                        "{}|{}|{}|{}", recursive.token_id, data_type.to_string(), "-",
                        recursive.token_id.len()
                    ),
                );
                data_type
            }
        }
    }
}
impl ToString for Data_Type {
    fn to_string(&self) -> String {
        match self {
            Data_Type::IntType(_) => "VAR_INT".to_string(),
            Data_Type::FloatType(_) => "VAR_FLOAT".to_string(),
            Data_Type::StringType(_) => "VAR_STRING".to_string(),
        }
    }
}
