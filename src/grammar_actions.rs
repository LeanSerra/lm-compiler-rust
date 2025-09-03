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
pub type TokenSum = String;
pub fn token_sum(_ctx: &Ctx, token: Token) -> TokenSum {
    token.value.into()
}
pub type TokenMul = String;
pub fn token_mul(_ctx: &Ctx, token: Token) -> TokenMul {
    token.value.into()
}
pub type TokenSub = String;
pub fn token_sub(_ctx: &Ctx, token: Token) -> TokenSub {
    token.value.into()
}
pub type TokenDiv = String;
pub fn token_div(_ctx: &Ctx, token: Token) -> TokenDiv {
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
pub type TokenWhile = String;
pub fn token_while(_ctx: &Ctx, token: Token) -> TokenWhile {
    token.value.into()
}
pub type TokenEqual = String;
pub fn token_equal(_ctx: &Ctx, token: Token) -> TokenEqual {
    token.value.into()
}
pub type TokenNotEqual = String;
pub fn token_not_equal(_ctx: &Ctx, token: Token) -> TokenNotEqual {
    token.value.into()
}
pub type TokenLess = String;
pub fn token_less(_ctx: &Ctx, token: Token) -> TokenLess {
    token.value.into()
}
pub type TokenLessEqual = String;
pub fn token_less_equal(_ctx: &Ctx, token: Token) -> TokenLessEqual {
    token.value.into()
}
pub type TokenGreater = String;
pub fn token_greater(_ctx: &Ctx, token: Token) -> TokenGreater {
    token.value.into()
}
pub type TokenGreaterEqual = String;
pub fn token_greater_equal(_ctx: &Ctx, token: Token) -> TokenGreaterEqual {
    token.value.into()
}
pub type TokenTrue = String;
pub fn token_true(_ctx: &Ctx, token: Token) -> TokenTrue {
    token.value.into()
}
pub type TokenFalse = String;
pub fn token_false(_ctx: &Ctx, token: Token) -> TokenFalse {
    token.value.into()
}
pub type TokenIf = String;
pub fn token_if(_ctx: &Ctx, token: Token) -> TokenIf {
    token.value.into()
}
pub type TokenElse = String;
pub fn token_else(_ctx: &Ctx, token: Token) -> TokenElse {
    token.value.into()
}
pub type TokenComma = String;
pub fn token_comma(_ctx: &Ctx, token: Token) -> TokenComma {
    token.value.into()
}
pub type TokenAnd = String;
pub fn token_and(_ctx: &Ctx, token: Token) -> TokenAnd {
    token.value.into()
}
pub type TokenOr = String;
pub fn token_or(_ctx: &Ctx, token: Token) -> TokenOr {
    token.value.into()
}
pub type TokenNot = String;
pub fn token_not(_ctx: &Ctx, token: Token) -> TokenNot {
    token.value.into()
}
pub type TokenRead = String;
pub fn token_read(_ctx: &Ctx, token: Token) -> TokenRead {
    token.value.into()
}
pub type TokenWrite = String;
pub fn token_write(_ctx: &Ctx, token: Token) -> TokenWrite {
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
pub struct FunctionRead {
    pub token_read: TokenRead,
    pub token_par_open: TokenParOpen,
    pub token_id: TokenId,
    pub token_par_close: TokenParClose,
}
pub fn function_read_function_read_call(
    _ctx: &Ctx,
    token_read: TokenRead,
    token_par_open: TokenParOpen,
    token_id: TokenId,
    token_par_close: TokenParClose,
) -> FunctionRead {
    FunctionRead {
        token_read,
        token_par_open,
        token_id,
        token_par_close,
    }
}
#[derive(Debug, Clone)]
pub struct FunctionWrite {
    pub token_write: TokenWrite,
    pub token_par_open: TokenParOpen,
    pub simple_expression: SimpleExpression,
    pub token_par_close: TokenParClose,
}
pub fn function_write_function_write_call(
    _ctx: &Ctx,
    token_write: TokenWrite,
    token_par_open: TokenParOpen,
    simple_expression: SimpleExpression,
    token_par_close: TokenParClose,
) -> FunctionWrite {
    FunctionWrite {
        token_write,
        token_par_open,
        simple_expression,
        token_par_close,
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
    VarDeclarationRead(FunctionRead),
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
pub fn var_declaration_var_declaration_read(
    _ctx: &Ctx,
    function_read: FunctionRead,
) -> VarDeclaration {
    VarDeclaration::VarDeclarationRead(function_read)
}
#[derive(Debug, Clone)]
pub struct ExpressionRecursive {
    pub statement: Statement,
    pub expressions: Box<Expressions>,
}
#[derive(Debug, Clone)]
pub enum Expressions {
    ExpressionSingle(Statement),
    ExpressionRecursive(ExpressionRecursive),
}
pub fn expressions_expression_single(_ctx: &Ctx, statement: Statement) -> Expressions {
    Expressions::ExpressionSingle(statement)
}
pub fn expressions_expression_recursive(
    _ctx: &Ctx,
    statement: Statement,
    expressions: Expressions,
) -> Expressions {
    Expressions::ExpressionRecursive(ExpressionRecursive {
        statement,
        expressions: Box::new(expressions),
    })
}
#[derive(Debug, Clone)]
pub enum Statement {
    StatementAssignment(Assignment),
    StatementIfStatement(Si),
    StatementElseStatement(Sino),
    StatementWhile(WhileLoop),
    StatementWrite(FunctionWrite),
}
pub fn statement_statement_assignment(_ctx: &Ctx, assignment: Assignment) -> Statement {
    Statement::StatementAssignment(assignment)
}
pub fn statement_statement_if_statement(_ctx: &Ctx, si: Si) -> Statement {
    Statement::StatementIfStatement(si)
}
pub fn statement_statement_else_statement(_ctx: &Ctx, sino: Sino) -> Statement {
    Statement::StatementElseStatement(sino)
}
pub fn statement_statement_while(_ctx: &Ctx, while_loop: WhileLoop) -> Statement {
    Statement::StatementWhile(while_loop)
}
pub fn statement_statement_write(
    _ctx: &Ctx,
    function_write: FunctionWrite,
) -> Statement {
    Statement::StatementWrite(function_write)
}
#[derive(Debug, Clone)]
pub struct Assignment {
    pub token_id: TokenId,
    pub token_assign: TokenAssign,
    pub simple_expression: SimpleExpression,
}
pub fn assignment_assignment(
    _ctx: &Ctx,
    token_id: TokenId,
    token_assign: TokenAssign,
    simple_expression: SimpleExpression,
) -> Assignment {
    Assignment {
        token_id,
        token_assign,
        simple_expression,
    }
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
#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub token_while: TokenWhile,
    pub token_par_open: TokenParOpen,
    pub boolean_expression: BooleanExpression,
    pub token_par_close: TokenParClose,
    pub token_cbopen: TokenCBOpen,
    pub body: Box<Body>,
    pub token_cbclose: TokenCBClose,
}
pub fn while_loop_while(
    _ctx: &Ctx,
    token_while: TokenWhile,
    token_par_open: TokenParOpen,
    boolean_expression: BooleanExpression,
    token_par_close: TokenParClose,
    token_cbopen: TokenCBOpen,
    body: Body,
    token_cbclose: TokenCBClose,
) -> WhileLoop {
    WhileLoop {
        token_while,
        token_par_open,
        boolean_expression,
        token_par_close,
        token_cbopen,
        body: Box::new(body),
        token_cbclose,
    }
}
#[derive(Debug, Clone)]
pub struct Si {
    pub token_if: TokenIf,
    pub token_par_open: TokenParOpen,
    pub boolean_expression: BooleanExpression,
    pub token_par_close: TokenParClose,
    pub token_cbopen: TokenCBOpen,
    pub body: Box<Body>,
    pub token_cbclose: TokenCBClose,
}
pub fn si_if_statement(
    _ctx: &Ctx,
    token_if: TokenIf,
    token_par_open: TokenParOpen,
    boolean_expression: BooleanExpression,
    token_par_close: TokenParClose,
    token_cbopen: TokenCBOpen,
    body: Body,
    token_cbclose: TokenCBClose,
) -> Si {
    Si {
        token_if,
        token_par_open,
        boolean_expression,
        token_par_close,
        token_cbopen,
        body: Box::new(body),
        token_cbclose,
    }
}
#[derive(Debug, Clone)]
pub struct Sino {
    pub token_else: TokenElse,
    pub token_cbopen: TokenCBOpen,
    pub body: Box<Body>,
    pub token_cbclose: TokenCBClose,
}
pub fn sino_else_statement(
    _ctx: &Ctx,
    token_else: TokenElse,
    token_cbopen: TokenCBOpen,
    body: Body,
    token_cbclose: TokenCBClose,
) -> Sino {
    Sino {
        token_else,
        token_cbopen,
        body: Box::new(body),
        token_cbclose,
    }
}
#[derive(Debug, Clone)]
pub struct BooleanExpressionSimpleExpression {
    pub simple_expression: SimpleExpression,
    pub boolean_expression_chain: BooleanExpressionChain,
}
#[derive(Debug, Clone)]
pub struct BooleanExpressionSimpleExpressionRecursive {
    pub simple_expression: SimpleExpression,
    pub boolean_expression_chain: BooleanExpressionChain,
    pub conjunction: Conjunction,
    pub boolean_expression: Box<BooleanExpression>,
}
#[derive(Debug, Clone)]
pub enum BooleanExpression {
    BooleanExpressionSimpleExpression(BooleanExpressionSimpleExpression),
    BooleanExpressionTrue(TokenTrue),
    BooleanExpressionFalse(TokenFalse),
    BooleanExpressionSimpleExpressionRecursive(
        BooleanExpressionSimpleExpressionRecursive,
    ),
    BooleanExpressionNotStatement(NotStatement),
}
pub fn boolean_expression_boolean_expression_simple_expression(
    _ctx: &Ctx,
    simple_expression: SimpleExpression,
    boolean_expression_chain: BooleanExpressionChain,
) -> BooleanExpression {
    BooleanExpression::BooleanExpressionSimpleExpression(BooleanExpressionSimpleExpression {
        simple_expression,
        boolean_expression_chain,
    })
}
pub fn boolean_expression_boolean_expression_true(
    _ctx: &Ctx,
    token_true: TokenTrue,
) -> BooleanExpression {
    BooleanExpression::BooleanExpressionTrue(token_true)
}
pub fn boolean_expression_boolean_expression_false(
    _ctx: &Ctx,
    token_false: TokenFalse,
) -> BooleanExpression {
    BooleanExpression::BooleanExpressionFalse(token_false)
}
pub fn boolean_expression_boolean_expression_simple_expression_recursive(
    _ctx: &Ctx,
    simple_expression: SimpleExpression,
    boolean_expression_chain: BooleanExpressionChain,
    conjunction: Conjunction,
    boolean_expression: BooleanExpression,
) -> BooleanExpression {
    BooleanExpression::BooleanExpressionSimpleExpressionRecursive(BooleanExpressionSimpleExpressionRecursive {
        simple_expression,
        boolean_expression_chain,
        conjunction,
        boolean_expression: Box::new(boolean_expression),
    })
}
pub fn boolean_expression_boolean_expression_not_statement(
    _ctx: &Ctx,
    not_statement: NotStatement,
) -> BooleanExpression {
    BooleanExpression::BooleanExpressionNotStatement(not_statement)
}
#[derive(Debug, Clone)]
pub struct BooleanExpressionChainNoO {
    pub comparison_op: ComparisonOp,
    pub simple_expression: SimpleExpression,
    pub boolean_expression_chain: Box<BooleanExpressionChain>,
}
pub type BooleanExpressionChain = Option<BooleanExpressionChainNoO>;
pub fn boolean_expression_chain_boolean_expression_chain_aux(
    _ctx: &Ctx,
    comparison_op: ComparisonOp,
    simple_expression: SimpleExpression,
    boolean_expression_chain: BooleanExpressionChain,
) -> BooleanExpressionChain {
    Some(BooleanExpressionChainNoO {
        comparison_op,
        simple_expression,
        boolean_expression_chain: Box::new(boolean_expression_chain),
    })
}
pub fn boolean_expression_chain_boolean_expression_chain_empty(
    _ctx: &Ctx,
) -> BooleanExpressionChain {
    None
}
#[derive(Debug, Clone)]
pub enum SimpleExpression {
    SimpleExpressionE(E),
    SimpleExpressionString(TokenStringLiteral),
}
pub fn simple_expression_simple_expression_e(_ctx: &Ctx, e: E) -> SimpleExpression {
    SimpleExpression::SimpleExpressionE(e)
}
pub fn simple_expression_simple_expression_string(
    _ctx: &Ctx,
    token_string_literal: TokenStringLiteral,
) -> SimpleExpression {
    SimpleExpression::SimpleExpressionString(token_string_literal)
}
#[derive(Debug, Clone)]
pub enum Conjunction {
    ConjunctionAnd(TokenAnd),
    ConjunctionOr(TokenOr),
}
pub fn conjunction_conjunction_and(_ctx: &Ctx, token_and: TokenAnd) -> Conjunction {
    Conjunction::ConjunctionAnd(token_and)
}
pub fn conjunction_conjunction_or(_ctx: &Ctx, token_or: TokenOr) -> Conjunction {
    Conjunction::ConjunctionOr(token_or)
}
#[derive(Debug, Clone)]
pub enum ComparisonOp {
    ComparisonOpEqual(TokenEqual),
    ComparisonOpNotEqual(TokenNotEqual),
    ComparisonOpLess(TokenLess),
    ComparisonOpLessEqual(TokenLessEqual),
    ComparisonOpGreater(TokenGreater),
    ComparisonOpGreaterEqual(TokenGreaterEqual),
}
pub fn comparison_op_comparison_op_equal(
    _ctx: &Ctx,
    token_equal: TokenEqual,
) -> ComparisonOp {
    ComparisonOp::ComparisonOpEqual(token_equal)
}
pub fn comparison_op_comparison_op_not_equal(
    _ctx: &Ctx,
    token_not_equal: TokenNotEqual,
) -> ComparisonOp {
    ComparisonOp::ComparisonOpNotEqual(token_not_equal)
}
pub fn comparison_op_comparison_op_less(
    _ctx: &Ctx,
    token_less: TokenLess,
) -> ComparisonOp {
    ComparisonOp::ComparisonOpLess(token_less)
}
pub fn comparison_op_comparison_op_less_equal(
    _ctx: &Ctx,
    token_less_equal: TokenLessEqual,
) -> ComparisonOp {
    ComparisonOp::ComparisonOpLessEqual(token_less_equal)
}
pub fn comparison_op_comparison_op_greater(
    _ctx: &Ctx,
    token_greater: TokenGreater,
) -> ComparisonOp {
    ComparisonOp::ComparisonOpGreater(token_greater)
}
pub fn comparison_op_comparison_op_greater_equal(
    _ctx: &Ctx,
    token_greater_equal: TokenGreaterEqual,
) -> ComparisonOp {
    ComparisonOp::ComparisonOpGreaterEqual(token_greater_equal)
}
#[derive(Debug, Clone)]
pub enum Number {
    NumberInt(TokenIntLiteral),
    NumberFloat(TokenFloatLiteral),
}
pub fn number_number_int(_ctx: &Ctx, token_int_literal: TokenIntLiteral) -> Number {
    Number::NumberInt(token_int_literal)
}
pub fn number_number_float(
    _ctx: &Ctx,
    token_float_literal: TokenFloatLiteral,
) -> Number {
    Number::NumberFloat(token_float_literal)
}
#[derive(Debug, Clone)]
pub struct NotStatement {
    pub token_not: TokenNot,
    pub boolean_expression: Box<BooleanExpression>,
}
pub fn not_statement_not(
    _ctx: &Ctx,
    token_not: TokenNot,
    boolean_expression: BooleanExpression,
) -> NotStatement {
    NotStatement {
        token_not,
        boolean_expression: Box::new(boolean_expression),
    }
}
#[derive(Debug, Clone)]
pub struct ESumT {
    pub e: Box<E>,
    pub token_sum: TokenSum,
    pub t: T,
}
#[derive(Debug, Clone)]
pub struct ESubT {
    pub e: Box<E>,
    pub token_sub: TokenSub,
    pub t: T,
}
#[derive(Debug, Clone)]
pub enum E {
    ESumT(ESumT),
    ESubT(ESubT),
    ET(T),
}
pub fn e_esum_t(_ctx: &Ctx, e: E, token_sum: TokenSum, t: T) -> E {
    E::ESumT(ESumT {
        e: Box::new(e),
        token_sum,
        t,
    })
}
pub fn e_esub_t(_ctx: &Ctx, e: E, token_sub: TokenSub, t: T) -> E {
    E::ESubT(ESubT {
        e: Box::new(e),
        token_sub,
        t,
    })
}
pub fn e_et(_ctx: &Ctx, t: T) -> E {
    E::ET(t)
}
#[derive(Debug, Clone)]
pub struct TMulF {
    pub t: Box<T>,
    pub token_mul: TokenMul,
    pub f: F,
}
#[derive(Debug, Clone)]
pub struct TDivF {
    pub t: Box<T>,
    pub token_div: TokenDiv,
    pub f: F,
}
#[derive(Debug, Clone)]
pub enum T {
    TMulF(TMulF),
    TDivF(TDivF),
    TF(F),
}
pub fn t_tmul_f(_ctx: &Ctx, t: T, token_mul: TokenMul, f: F) -> T {
    T::TMulF(TMulF {
        t: Box::new(t),
        token_mul,
        f,
    })
}
pub fn t_tdiv_f(_ctx: &Ctx, t: T, token_div: TokenDiv, f: F) -> T {
    T::TDivF(TDivF {
        t: Box::new(t),
        token_div,
        f,
    })
}
pub fn t_tf(_ctx: &Ctx, f: F) -> T {
    T::TF(f)
}
#[derive(Debug, Clone)]
pub struct FParen {
    pub token_par_open: TokenParOpen,
    pub e: Box<E>,
    pub token_par_close: TokenParClose,
}
#[derive(Debug, Clone)]
pub enum F {
    FId(TokenId),
    FNumber(Number),
    FParen(FParen),
}
pub fn f_fid(_ctx: &Ctx, token_id: TokenId) -> F {
    F::FId(token_id)
}
pub fn f_fnumber(_ctx: &Ctx, number: Number) -> F {
    F::FNumber(number)
}
pub fn f_fparen(
    _ctx: &Ctx,
    token_par_open: TokenParOpen,
    e: E,
    token_par_close: TokenParClose,
) -> F {
    F::FParen(FParen {
        token_par_open,
        e: Box::new(e),
        token_par_close,
    })
}
