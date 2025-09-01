/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use super::grammar::{TokenKind, Context};
use super::grammar_lexer::Input;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type TokenInt = String;
pub fn token_int(_ctx: &Ctx, token: Token) -> TokenInt {
    token.value.into()
}
pub type TokenFloat = String;
pub fn token_float(_ctx: &Ctx, token: Token) -> TokenFloat {
    token.value.into()
}
pub type TokenString = String;
pub fn token_string(_ctx: &Ctx, token: Token) -> TokenString {
    token.value.into()
}
pub type TokenIntLiteral = String;
pub fn token_int_literal(_ctx: &Ctx, token: Token) -> TokenIntLiteral {
    token.value.into()
}
pub type TokenFloatLiteral = String;
pub fn token_float_literal(_ctx: &Ctx, token: Token) -> TokenFloatLiteral {
    token.value.into()
}
pub type TokenStringLiteral = String;
pub fn token_string_literal(_ctx: &Ctx, token: Token) -> TokenStringLiteral {
    token.value.into()
}
pub type TokenId = String;
pub fn token_id(_ctx: &Ctx, token: Token) -> TokenId {
    token.value.into()
}
pub type TokenAssign = String;
pub fn token_assign(_ctx: &Ctx, token: Token) -> TokenAssign {
    token.value.into()
}
pub type TokenParOpen = String;
pub fn token_par_open(_ctx: &Ctx, token: Token) -> TokenParOpen {
    token.value.into()
}
pub type TokenParClose = String;
pub fn token_par_close(_ctx: &Ctx, token: Token) -> TokenParClose {
    token.value.into()
}
pub type TokenCBOpen = String;
pub fn token_cbopen(_ctx: &Ctx, token: Token) -> TokenCBOpen {
    token.value.into()
}
pub type TokenCBClose = String;
pub fn token_cbclose(_ctx: &Ctx, token: Token) -> TokenCBClose {
    token.value.into()
}
pub type TokenColon = String;
pub fn token_colon(_ctx: &Ctx, token: Token) -> TokenColon {
    token.value.into()
}
pub type TokenInit = String;
pub fn token_init(_ctx: &Ctx, token: Token) -> TokenInit {
    token.value.into()
}
pub type TokenComa = String;
pub fn token_coma(_ctx: &Ctx, token: Token) -> TokenComa {
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
pub fn program_c1(
    _ctx: &Ctx,
    token_id: TokenId,
    token_par_open: TokenParOpen,
    token_par_close: TokenParClose,
    token_cbopen: TokenCBOpen,
    body: Body,
    token_cbclose: TokenCBClose,
) -> Program {
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
pub struct BodyC1 {
    pub token_init: TokenInit,
    pub init_body: InitBody,
    pub expressions: Expressions,
}
#[derive(Debug, Clone)]
pub struct BodyC2 {
    pub token_init: TokenInit,
    pub init_body: InitBody,
}
pub type Body = Option<BodyNoO>;
#[derive(Debug, Clone)]
pub enum BodyNoO {
    C1(BodyC1),
    C2(BodyC2),
}
pub fn body_c1(
    _ctx: &Ctx,
    token_init: TokenInit,
    init_body: InitBody,
    expressions: Expressions,
) -> Body {
    Some(
        BodyNoO::C1(BodyC1 {
            token_init,
            init_body,
            expressions,
        }),
    )
}
pub fn body_c2(_ctx: &Ctx, token_init: TokenInit, init_body: InitBody) -> Body {
    Some(BodyNoO::C2(BodyC2 { token_init, init_body }))
}
pub fn body_empty(_ctx: &Ctx) -> Body {
    None
}
#[derive(Debug, Clone)]
pub struct InitBody {
    pub token_cbopen: TokenCBOpen,
    pub var_declarations: VarDeclarations,
    pub token_cbclose: TokenCBClose,
}
pub fn init_body_c1(
    _ctx: &Ctx,
    token_cbopen: TokenCBOpen,
    var_declarations: VarDeclarations,
    token_cbclose: TokenCBClose,
) -> InitBody {
    InitBody {
        token_cbopen,
        var_declarations,
        token_cbclose,
    }
}
#[derive(Debug, Clone)]
pub struct VarDeclarationsC2 {
    pub var_declaration: VarDeclaration,
    pub var_declarations: Box<VarDeclarations>,
}
#[derive(Debug, Clone)]
pub enum VarDeclarations {
    VarDeclaration(VarDeclaration),
    C2(VarDeclarationsC2),
}
pub fn var_declarations_var_declaration(
    _ctx: &Ctx,
    var_declaration: VarDeclaration,
) -> VarDeclarations {
    VarDeclarations::VarDeclaration(var_declaration)
}
pub fn var_declarations_c2(
    _ctx: &Ctx,
    var_declaration: VarDeclaration,
    var_declarations: VarDeclarations,
) -> VarDeclarations {
    VarDeclarations::C2(VarDeclarationsC2 {
        var_declaration,
        var_declarations: Box::new(var_declarations),
    })
}
#[derive(Debug, Clone)]
pub struct VarDeclarationC1 {
    pub token_id: TokenId,
    pub token_colon: TokenColon,
    pub data_type: Data_Type,
}
#[derive(Debug, Clone)]
pub struct VarDeclarationC2 {
    pub token_id: TokenId,
    pub token_coma: TokenComa,
    pub var_declaration: Box<VarDeclaration>,
}
#[derive(Debug, Clone)]
pub enum VarDeclaration {
    C1(VarDeclarationC1),
    C2(VarDeclarationC2),
}
pub fn var_declaration_c1(
    _ctx: &Ctx,
    token_id: TokenId,
    token_colon: TokenColon,
    data_type: Data_Type,
) -> VarDeclaration {
    VarDeclaration::C1(VarDeclarationC1 {
        token_id,
        token_colon,
        data_type,
    })
}
pub fn var_declaration_c2(
    _ctx: &Ctx,
    token_id: TokenId,
    token_coma: TokenComa,
    var_declaration: VarDeclaration,
) -> VarDeclaration {
    VarDeclaration::C2(VarDeclarationC2 {
        token_id,
        token_coma,
        var_declaration: Box::new(var_declaration),
    })
}
#[derive(Debug, Clone)]
pub struct ExpressionsC2 {
    pub expression: Expression,
    pub expressions: Box<Expressions>,
}
#[derive(Debug, Clone)]
pub enum Expressions {
    Expression(Expression),
    C2(ExpressionsC2),
}
pub fn expressions_expression(_ctx: &Ctx, expression: Expression) -> Expressions {
    Expressions::Expression(expression)
}
pub fn expressions_c2(
    _ctx: &Ctx,
    expression: Expression,
    expressions: Expressions,
) -> Expressions {
    Expressions::C2(ExpressionsC2 {
        expression,
        expressions: Box::new(expressions),
    })
}
pub type Expression = Assignment;
pub fn expression_assignment(_ctx: &Ctx, assignment: Assignment) -> Expression {
    assignment
}
#[derive(Debug, Clone)]
pub struct Assignment {
    pub token_id: TokenId,
    pub token_assign: TokenAssign,
    pub literal: Literal,
}
pub fn assignment_c1(
    _ctx: &Ctx,
    token_id: TokenId,
    token_assign: TokenAssign,
    literal: Literal,
) -> Assignment {
    Assignment {
        token_id,
        token_assign,
        literal,
    }
}
#[derive(Debug, Clone)]
pub enum Literal {
    TokenIntLiteral(TokenIntLiteral),
    TokenFloatLiteral(TokenFloatLiteral),
    TokenStringLiteral(TokenStringLiteral),
}
pub fn literal_token_int_literal(
    _ctx: &Ctx,
    token_int_literal: TokenIntLiteral,
) -> Literal {
    Literal::TokenIntLiteral(token_int_literal)
}
pub fn literal_token_float_literal(
    _ctx: &Ctx,
    token_float_literal: TokenFloatLiteral,
) -> Literal {
    Literal::TokenFloatLiteral(token_float_literal)
}
pub fn literal_token_string_literal(
    _ctx: &Ctx,
    token_string_literal: TokenStringLiteral,
) -> Literal {
    Literal::TokenStringLiteral(token_string_literal)
}
#[derive(Debug, Clone)]
pub enum Data_Type {
    TokenInt(TokenInt),
    TokenFloat(TokenFloat),
    TokenString(TokenString),
}
pub fn data_type_token_int(_ctx: &Ctx, token_int: TokenInt) -> Data_Type {
    Data_Type::TokenInt(token_int)
}
pub fn data_type_token_float(_ctx: &Ctx, token_float: TokenFloat) -> Data_Type {
    Data_Type::TokenFloat(token_float)
}
pub fn data_type_token_string(_ctx: &Ctx, token_string: TokenString) -> Data_Type {
    Data_Type::TokenString(token_string)
}
