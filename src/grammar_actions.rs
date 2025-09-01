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
pub type TokenComma = String;
pub fn token_comma(_ctx: &Ctx, token: Token) -> TokenComma {
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
pub struct BodyInitExpressions {
    pub token_init: TokenInit,
    pub init_body: InitBody,
    pub expressions: Expressions,
}
#[derive(Debug, Clone)]
pub struct BodyInit {
    pub token_init: TokenInit,
    pub init_body: InitBody,
}
pub type Body = Option<BodyNoO>;
#[derive(Debug, Clone)]
pub enum BodyNoO {
    BodyInitExpressions(BodyInitExpressions),
    BodyInit(BodyInit),
    BodyExpressions(Expressions),
}
pub fn body_body_init_expressions(
    _ctx: &Ctx,
    token_init: TokenInit,
    init_body: InitBody,
    expressions: Expressions,
) -> Body {
    Some(
        BodyNoO::BodyInitExpressions(BodyInitExpressions {
            token_init,
            init_body,
            expressions,
        }),
    )
}
pub fn body_body_init(_ctx: &Ctx, token_init: TokenInit, init_body: InitBody) -> Body {
    Some(BodyNoO::BodyInit(BodyInit { token_init, init_body }))
}
pub fn body_body_expressions(_ctx: &Ctx, expressions: Expressions) -> Body {
    Some(BodyNoO::BodyExpressions(expressions))
}
pub fn body_body_empty(_ctx: &Ctx) -> Body {
    None
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
    VarDeclarations::VarDeclarationsSingle(var_declaration)
}
pub fn var_declarations_var_declarations_recursive(
    _ctx: &Ctx,
    var_declaration: VarDeclaration,
    var_declarations: VarDeclarations,
) -> VarDeclarations {
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
    Expressions::ExpressionSingle(expression)
}
pub fn expressions_expression_recursive(
    _ctx: &Ctx,
    expression: Expression,
    expressions: Expressions,
) -> Expressions {
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
    Literal::IntegerLiteral(token_int_literal)
}
pub fn literal_float_literal(
    _ctx: &Ctx,
    token_float_literal: TokenFloatLiteral,
) -> Literal {
    Literal::FloatLiteral(token_float_literal)
}
pub fn literal_string_literal(
    _ctx: &Ctx,
    token_string_literal: TokenStringLiteral,
) -> Literal {
    Literal::StringLiteral(token_string_literal)
}
#[derive(Debug, Clone)]
pub enum Data_Type {
    IntType(TokenInt),
    FloatType(TokenFloat),
    StringType(TokenString),
}
pub fn data_type_int_type(_ctx: &Ctx, token_int: TokenInt) -> Data_Type {
    Data_Type::IntType(token_int)
}
pub fn data_type_float_type(_ctx: &Ctx, token_float: TokenFloat) -> Data_Type {
    Data_Type::FloatType(token_float)
}
pub fn data_type_string_type(_ctx: &Ctx, token_string: TokenString) -> Data_Type {
    Data_Type::StringType(token_string)
}
