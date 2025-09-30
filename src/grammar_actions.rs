use super::grammar::{Context, TokenKind};
use super::grammar_lexer::Input;
use crate::context::{write_to_lexer_file, write_to_parser_file, write_to_symbol_table_file};
use crate::grammar_lexer::log_error;
use crate::read_source_to_string;
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::{Context as RustemoContext, Input as RustemoInput, Token as RustemoToken};
use std::fmt::Display;
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
pub type TokenIntLiteral = u64;
pub fn token_int_literal(ctx: &Ctx, token: Token) -> TokenIntLiteral {
    write_to_lexer_file(&format!("INT_LITERAL: {}", token.value));
    match token.value.parse() {
        Err(e) => {
            log_error(
                ctx.range(),
                crate::CompilerError::Parser(format!("Invalid integer literal {e}")),
                0,
                &read_source_to_string().unwrap(),
            );
            std::process::exit(1)
        }
        Ok(value) => value,
    }
}
#[derive(Debug, Clone)]
pub struct TokenFloatLiteral {
    original: String,
    parsed: f64,
}
pub fn token_float_literal(ctx: &Ctx, token: Token) -> TokenFloatLiteral {
    write_to_lexer_file(&format!("FLOAT_LITERAL: {}", token.value));
    let value: f64 = match token.value.parse() {
        Err(e) => {
            log_error(
                ctx.range(),
                crate::CompilerError::Parser(format!("Invalid float literal {e}")),
                0,
                &read_source_to_string().unwrap(),
            );
            std::process::exit(1)
        }
        Ok(value) => value,
    };
    if value.is_subnormal() {
        log_error(
            ctx.range(),
            crate::CompilerError::Parser(format!("Invalid float literal because its: {value}")),
            0,
            &read_source_to_string().unwrap(),
        );
    }
    TokenFloatLiteral {
        original: token.value.to_string(),
        parsed: value,
    }
}
pub type TokenStringLiteral = String;
/// Parses a TokenStringLiteral by removing the "" and returning an owned string
pub fn token_string_literal(_ctx: &Ctx, mut token: Token) -> TokenStringLiteral {
    token.value = token.value.slice(1..token.value.len() - 1);
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
pub type TokenSum = String;
pub fn token_sum(_ctx: &Ctx, token: Token) -> TokenSum {
    write_to_lexer_file(&format!("SUM: {}", token.value));
    token.value.into()
}
pub type TokenMul = String;
pub fn token_mul(_ctx: &Ctx, token: Token) -> TokenMul {
    write_to_lexer_file(&format!("MUL: {}", token.value));
    token.value.into()
}
pub type TokenSub = String;
pub fn token_sub(_ctx: &Ctx, token: Token) -> TokenSub {
    write_to_lexer_file(&format!("SUB: {}", token.value));
    token.value.into()
}
pub type TokenDiv = String;
pub fn token_div(_ctx: &Ctx, token: Token) -> TokenDiv {
    write_to_lexer_file(&format!("DIV: {}", token.value));
    token.value.into()
}
pub type TokenParOpen = String;
pub fn token_par_open(_ctx: &Ctx, token: Token) -> TokenParOpen {
    write_to_lexer_file(&format!("PAR_OPEN: {}", token.value));
    token.value.into()
}
pub type TokenParClose = String;
pub fn token_par_close(_ctx: &Ctx, token: Token) -> TokenParClose {
    write_to_lexer_file(&format!("PAR_CLOSE: {}", token.value));
    token.value.into()
}
pub type TokenCBOpen = String;
pub fn token_cbopen(_ctx: &Ctx, token: Token) -> TokenCBOpen {
    write_to_lexer_file(&format!("CB_OPEN: {}", token.value));
    token.value.into()
}
pub type TokenCBClose = String;
pub fn token_cbclose(_ctx: &Ctx, token: Token) -> TokenCBClose {
    write_to_lexer_file(&format!("CB_CLOSE: {}", token.value));
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
pub type TokenWhile = String;
pub fn token_while(_ctx: &Ctx, token: Token) -> TokenWhile {
    write_to_lexer_file(&format!("WHILE: {}", token.value));
    token.value.into()
}
pub type TokenEqual = String;
pub fn token_equal(_ctx: &Ctx, token: Token) -> TokenEqual {
    write_to_lexer_file(&format!("EQUAL: {}", token.value));
    token.value.into()
}
pub type TokenNotEqual = String;
pub fn token_not_equal(_ctx: &Ctx, token: Token) -> TokenNotEqual {
    write_to_lexer_file(&format!("NOT_EQUAL: {}", token.value));
    token.value.into()
}
pub type TokenLess = String;
pub fn token_less(_ctx: &Ctx, token: Token) -> TokenLess {
    write_to_lexer_file(&format!("LESS: {}", token.value));
    token.value.into()
}
pub type TokenLessEqual = String;
pub fn token_less_equal(_ctx: &Ctx, token: Token) -> TokenLessEqual {
    write_to_lexer_file(&format!("LESS_EQUAL: {}", token.value));
    token.value.into()
}
pub type TokenGreater = String;
pub fn token_greater(_ctx: &Ctx, token: Token) -> TokenGreater {
    write_to_lexer_file(&format!("GREATER: {}", token.value));
    token.value.into()
}
pub type TokenGreaterEqual = String;
pub fn token_greater_equal(_ctx: &Ctx, token: Token) -> TokenGreaterEqual {
    write_to_lexer_file(&format!("GREATER_EQUAL: {}", token.value));
    token.value.into()
}
pub type TokenTrue = String;
pub fn token_true(_ctx: &Ctx, token: Token) -> TokenTrue {
    write_to_lexer_file(&format!("TRUE: {}", token.value));
    token.value.into()
}
pub type TokenFalse = String;
pub fn token_false(_ctx: &Ctx, token: Token) -> TokenFalse {
    write_to_lexer_file(&format!("FALSE: {}", token.value));
    token.value.into()
}
pub type TokenIf = String;
pub fn token_if(_ctx: &Ctx, token: Token) -> TokenIf {
    write_to_lexer_file(&format!("IF: {}", token.value));
    token.value.into()
}
pub type TokenElse = String;
pub fn token_else(_ctx: &Ctx, token: Token) -> TokenElse {
    write_to_lexer_file(&format!("ELSE: {}", token.value));
    token.value.into()
}
pub type TokenComma = String;
pub fn token_comma(_ctx: &Ctx, token: Token) -> TokenComma {
    write_to_lexer_file(&format!("COMMA: {}", token.value));
    token.value.into()
}
pub type TokenAnd = String;
pub fn token_and(_ctx: &Ctx, token: Token) -> TokenAnd {
    write_to_lexer_file(&format!("AND: {}", token.value));
    token.value.into()
}
pub type TokenOr = String;
pub fn token_or(_ctx: &Ctx, token: Token) -> TokenOr {
    write_to_lexer_file(&format!("OR: {}", token.value));
    token.value.into()
}
pub type TokenNot = String;
pub fn token_not(_ctx: &Ctx, token: Token) -> TokenNot {
    write_to_lexer_file(&format!("NOT: {}", token.value));
    token.value.into()
}
pub type TokenRead = String;
pub fn token_read(_ctx: &Ctx, token: Token) -> TokenRead {
    write_to_lexer_file(&format!("READ: {}", token.value));
    token.value.into()
}
pub type TokenWrite = String;
pub fn token_write(_ctx: &Ctx, token: Token) -> TokenWrite {
    write_to_lexer_file(&format!("WRITE: {}", token.value));
    token.value.into()
}
pub type TokenIsZero = String;
pub fn token_is_zero(_ctx: &Ctx, token: Token) -> TokenIsZero {
    write_to_lexer_file(&format!("IS_ZERO: {}", token.value));
    token.value.into()
}
pub type TokenConvDate = String;
pub fn token_conv_date(_ctx: &Ctx, token: Token) -> TokenConvDate {
    write_to_lexer_file(&format!("CONV_DATE: {}", token.value));
    token.value.into()
}
pub type TokenDate = String;
pub fn token_date(_ctx: &Ctx, token: Token) -> TokenDate {
    write_to_lexer_file(&format!("DATE: {}", token.value));
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct ProgramWithMain {
    pub token_id: TokenId,
    pub token_par_open: TokenParOpen,
    pub token_par_close: TokenParClose,
    pub token_cbopen: TokenCBOpen,
    pub body: Body,
    pub token_cbclose: TokenCBClose,
}
#[derive(Debug, Clone)]
pub enum Program {
    ProgramWithMain(ProgramWithMain),
    ProgramOnlyBody(Body),
}
pub fn program_program_with_main(
    _ctx: &Ctx,
    token_id: TokenId,
    token_par_open: TokenParOpen,
    token_par_close: TokenParClose,
    token_cbopen: TokenCBOpen,
    body: Body,
    token_cbclose: TokenCBClose,
) -> Program {
    write_to_parser_file(&format!(
        "<Program> -> {token_id} {token_par_open} {token_par_close} {token_cbopen} <Body> {token_cbclose}"
    ));
    Program::ProgramWithMain(ProgramWithMain {
        token_id,
        token_par_open,
        token_par_close,
        token_cbopen,
        body,
        token_cbclose,
    })
}
pub fn program_program_only_body(_ctx: &Ctx, body: Body) -> Program {
    write_to_parser_file(&format!("<Program> -> <Body>"));
    Program::ProgramOnlyBody(body)
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
    write_to_parser_file(&format!("<Body> -> {token_init} <InitBody> <Expressions>"));
    Some(BodyNoO::BodyInitExpressions(BodyInitExpressions {
        token_init,
        init_body,
        expressions,
    }))
}
pub fn body_body_init(_ctx: &Ctx, token_init: TokenInit, init_body: InitBody) -> Body {
    write_to_parser_file(&format!("<Body> -> {token_init} <InitBody>"));
    Some(BodyNoO::BodyInit(BodyInit {
        token_init,
        init_body,
    }))
}
pub fn body_body_expressions(_ctx: &Ctx, expressions: Expressions) -> Body {
    ///write_to_parser_file(&format!("<Body> -> <Expressions>"));
    Some(BodyNoO::BodyExpressions(expressions))
}
pub fn body_body_empty(_ctx: &Ctx) -> Body {
    write_to_parser_file("<Body> -> EMPTY");
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
    write_to_parser_file(&format!(
        "<InitBody> -> {token_cbopen} <VarDeclarations> {token_cbclose}"
    ));
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
    write_to_parser_file(&format!(
        "<FunctionRead> -> {token_read} {token_par_open} {token_id} {token_par_close}"
    ));
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
    write_to_parser_file(&format!(
        "<FunctionWrite> -> {token_write} {token_par_open} <SimpleExpression> {token_par_close}"
    ));
    FunctionWrite {
        token_write,
        token_par_open,
        simple_expression,
        token_par_close,
    }
}
#[derive(Debug, Clone)]
pub struct FunctionIsZero {
    pub token_is_zero: TokenIsZero,
    pub token_par_open: TokenParOpen,
    pub arithmetic_expression: ArithmeticExpression,
    pub token_par_close: TokenParClose,
}
pub fn function_is_zero_function_is_zero_call(
    _ctx: &Ctx,
    token_is_zero: TokenIsZero,
    token_par_open: TokenParOpen,
    arithmetic_expression: ArithmeticExpression,
    token_par_close: TokenParClose,
) -> FunctionIsZero {
    write_to_parser_file(&format!(
        "<FunctionIsZero> -> {token_is_zero} {token_par_open} <E> {token_par_close}"
    ));
    FunctionIsZero {
        token_is_zero,
        token_par_open,
        arithmetic_expression,
        token_par_close,
    }
}
#[derive(Debug, Clone)]
pub struct FunctionConvDate {
    pub token_conv_date: TokenConvDate,
    pub token_par_open: TokenParOpen,
    pub token_date: TokenDate,
    pub token_par_close: TokenParClose,
}
pub fn function_conv_date_function_conv_date_variable_call(
    _ctx: &Ctx,
    token_conv_date: TokenConvDate,
    token_par_open: TokenParOpen,
    token_date: TokenDate,
    token_par_close: TokenParClose,
) -> FunctionConvDate {
    write_to_parser_file(&format!(
        "<FunctionConvDate> -> {token_conv_date} {token_par_open} {token_date} {token_par_close}"
    ));
    FunctionConvDate {
        token_conv_date,
        token_par_open,
        token_date,
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
    var_declaration.write_to_symbol_table();
    ///write_to_parser_file(&format!("<VarDeclarations> -> <VarDeclaration>"));
    VarDeclarations::VarDeclarationsSingle(var_declaration)
}
pub fn var_declarations_var_declarations_recursive(
    _ctx: &Ctx,
    var_declaration: VarDeclaration,
    var_declarations: VarDeclarations,
) -> VarDeclarations {
    var_declaration.write_to_symbol_table();
    ///write_to_parser_file(&format!("<VarDeclarations> -> <VarDeclaration> <VarDeclarations>"));
    VarDeclarations::VarDeclarationsRecursive(VarDeclarationsRecursive {
        var_declaration,
        var_declarations: Box::new(var_declarations),
    })
}
#[derive(Debug, Clone)]
pub struct VarDeclarationSingle {
    pub token_id: TokenId,
    pub token_colon: TokenColon,
    pub data_type: DataType,
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
    data_type: DataType,
) -> VarDeclaration {
    write_to_parser_file(&format!(
        "<VarDeclaration> -> {token_id} {token_colon} <DataType>"
    ));
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
    write_to_parser_file(&format!(
        "<VarDeclaration> -> {token_id} {token_comma} <VarDeclaration>"
    ));
    VarDeclaration::VarDeclarationRecursive(VarDeclarationRecursive {
        token_id,
        token_comma,
        var_declaration: Box::new(var_declaration),
    })
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
    ///write_to_parser_file(&format!("<Expressions> -> <Statement>"));
    Expressions::ExpressionSingle(statement)
}
pub fn expressions_expression_recursive(
    _ctx: &Ctx,
    statement: Statement,
    expressions: Expressions,
) -> Expressions {
    ///write_to_parser_file(&format!("<Expressions> -> <Statement> <Expressions>"));
    Expressions::ExpressionRecursive(ExpressionRecursive {
        statement,
        expressions: Box::new(expressions),
    })
}
#[derive(Debug, Clone)]
pub enum Statement {
    StatementAssignment(Assignment),
    StatementIfStatement(IfStatement),
    StatementElseStatement(ElseStatement),
    StatementWhile(WhileLoop),
    StatementWrite(FunctionWrite),
    StatementRead(FunctionRead),
    StatementConvDate(FunctionConvDate),
}
pub fn statement_statement_assignment(_ctx: &Ctx, assignment: Assignment) -> Statement {
    ///write_to_parser_file(&format!("<Statement> -> <Assignment>"));
    Statement::StatementAssignment(assignment)
}
pub fn statement_statement_if_statement(_ctx: &Ctx, if_statement: IfStatement) -> Statement {
    ///write_to_parser_file(&format!("<Statement> -> <IfStatement>"));
    Statement::StatementIfStatement(if_statement)
}
pub fn statement_statement_else_statement(_ctx: &Ctx, else_statement: ElseStatement) -> Statement {
    ///write_to_parser_file(&format!("<Statement> -> <ElseStatement>"));
    Statement::StatementElseStatement(else_statement)
}
pub fn statement_statement_while(_ctx: &Ctx, while_loop: WhileLoop) -> Statement {
    ///write_to_parser_file(&format!("<Statement> -> <WhileLoop>"));
    Statement::StatementWhile(while_loop)
}
pub fn statement_statement_write(_ctx: &Ctx, function_write: FunctionWrite) -> Statement {
    ///write_to_parser_file(&format!("<Statement> -> <FunctionWrite>"));
    Statement::StatementWrite(function_write)
}
pub fn statement_statement_read(_ctx: &Ctx, function_read: FunctionRead) -> Statement {
    ///write_to_parser_file(&format!("<Statement> -> <FunctionRead>"));
    Statement::StatementRead(function_read)
}
pub fn statement_statement_conv_date(
    _ctx: &Ctx,
    function_conv_date: FunctionConvDate,
) -> Statement {
    ///write_to_parser_file(&format!("<Statement> -> <FunctionConvDate>"));
    Statement::StatementConvDate(function_conv_date)
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
    write_to_parser_file(&format!(
        "<Assignment> -> {token_id} {token_assign} <SimpleExpression>"
    ));
    Assignment {
        token_id,
        token_assign,
        simple_expression,
    }
}
#[derive(Debug, Clone)]
pub enum DataType {
    IntType(TokenInt),
    FloatType(TokenFloat),
    StringType(TokenString),
}
pub fn data_type_int_type(_ctx: &Ctx, token_int: TokenInt) -> DataType {
    write_to_parser_file(&format!("<DataType> -> {token_int}"));
    DataType::IntType(token_int)
}
pub fn data_type_float_type(_ctx: &Ctx, token_float: TokenFloat) -> DataType {
    write_to_parser_file(&format!("<DataType> -> {token_float}"));
    DataType::FloatType(token_float)
}
pub fn data_type_string_type(_ctx: &Ctx, token_string: TokenString) -> DataType {
    write_to_parser_file(&format!("<DataType> -> {token_string}"));
    DataType::StringType(token_string)
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
    write_to_parser_file(&format!(
        "<WhileLoop> -> {token_while} {token_par_open} <BooleanExpression> {token_par_close} {token_cbopen} <Body> {token_cbclose}"
    ));
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
pub struct IfStatement {
    pub token_if: TokenIf,
    pub token_par_open: TokenParOpen,
    pub boolean_expression: BooleanExpression,
    pub token_par_close: TokenParClose,
    pub token_cbopen: TokenCBOpen,
    pub body: Box<Body>,
    pub token_cbclose: TokenCBClose,
}
pub fn if_statement_if_statement(
    _ctx: &Ctx,
    token_if: TokenIf,
    token_par_open: TokenParOpen,
    boolean_expression: BooleanExpression,
    token_par_close: TokenParClose,
    token_cbopen: TokenCBOpen,
    body: Body,
    token_cbclose: TokenCBClose,
) -> IfStatement {
    write_to_parser_file(&format!(
        "<IfStatement> -> {token_if} {token_par_open} <BooleanExpression> {token_par_close} {token_cbopen} <Body> {token_cbclose}"
    ));
    IfStatement {
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
pub struct ElseStatement {
    pub token_else: TokenElse,
    pub token_cbopen: TokenCBOpen,
    pub body: Box<Body>,
    pub token_cbclose: TokenCBClose,
}
pub fn else_statement_else_statement(
    _ctx: &Ctx,
    token_else: TokenElse,
    token_cbopen: TokenCBOpen,
    body: Body,
    token_cbclose: TokenCBClose,
) -> ElseStatement {
    write_to_parser_file(&format!(
        "<ElseStatement> -> {token_else} {token_cbopen} <Body> {token_cbclose}"
    ));
    ElseStatement {
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
    BooleanExpressionSimpleExpressionRecursive(BooleanExpressionSimpleExpressionRecursive),
    BooleanExpressionNotStatement(NotStatement),
    BooleanExpressionIsZero(FunctionIsZero),
}
pub fn boolean_expression_boolean_expression_simple_expression(
    _ctx: &Ctx,
    simple_expression: SimpleExpression,
    boolean_expression_chain: BooleanExpressionChain,
) -> BooleanExpression {
    ///write_to_parser_file(
    ///    &format!("<BooleanExpression> -> <SimpleExpression> <BooleanExpressionChain>"),
    ///);
    BooleanExpression::BooleanExpressionSimpleExpression(BooleanExpressionSimpleExpression {
        simple_expression,
        boolean_expression_chain,
    })
}
pub fn boolean_expression_boolean_expression_true(
    _ctx: &Ctx,
    token_true: TokenTrue,
) -> BooleanExpression {
    write_to_parser_file(&format!("<BooleanExpression> -> {token_true}"));
    BooleanExpression::BooleanExpressionTrue(token_true)
}
pub fn boolean_expression_boolean_expression_false(
    _ctx: &Ctx,
    token_false: TokenFalse,
) -> BooleanExpression {
    write_to_parser_file(&format!("<BooleanExpression> -> {token_false}"));
    BooleanExpression::BooleanExpressionFalse(token_false)
}
pub fn boolean_expression_boolean_expression_simple_expression_recursive(
    _ctx: &Ctx,
    simple_expression: SimpleExpression,
    boolean_expression_chain: BooleanExpressionChain,
    conjunction: Conjunction,
    boolean_expression: BooleanExpression,
) -> BooleanExpression {
    write_to_parser_file(&format!(
        "<BooleanExpression> -> <SimpleExpression> <BooleanExpressionChain> <Conjunction> <BooleanExpression>"
    ));
    BooleanExpression::BooleanExpressionSimpleExpressionRecursive(
        BooleanExpressionSimpleExpressionRecursive {
            simple_expression,
            boolean_expression_chain,
            conjunction,
            boolean_expression: Box::new(boolean_expression),
        },
    )
}
pub fn boolean_expression_boolean_expression_not_statement(
    _ctx: &Ctx,
    not_statement: NotStatement,
) -> BooleanExpression {
    ///write_to_parser_file(&format!("<BooleanExpression> -> <NotStatement>"));
    BooleanExpression::BooleanExpressionNotStatement(not_statement)
}
pub fn boolean_expression_boolean_expression_is_zero(
    _ctx: &Ctx,
    function_is_zero: FunctionIsZero,
) -> BooleanExpression {
    ///write_to_parser_file(&format!("<BooleanExpression> -> <FunctionIsZero>"));
    BooleanExpression::BooleanExpressionIsZero(function_is_zero)
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
    ///write_to_parser_file(
    ///    &format!(
    ///        "<BooleanExpressionChain> -> <ComparisonOp> <SimpleExpression> <BooleanExpressionChain>"
    ///    ),
    ///);
    Some(BooleanExpressionChainNoO {
        comparison_op,
        simple_expression,
        boolean_expression_chain: Box::new(boolean_expression_chain),
    })
}
pub fn boolean_expression_chain_boolean_expression_chain_empty(
    _ctx: &Ctx,
) -> BooleanExpressionChain {
    write_to_parser_file("<BooleanExpressionChain> -> EMPTY");
    None
}
#[derive(Debug, Clone)]
pub enum SimpleExpression {
    SimpleExpressionArithmeticExpression(ArithmeticExpression),
    SimpleExpressionString(TokenStringLiteral),
}
pub fn simple_expression_simple_expression_arithmetic(
    _ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
) -> SimpleExpression {
    ///write_to_parser_file(&format!("<SimpleExpression> -> <E>"));
    SimpleExpression::SimpleExpressionArithmeticExpression(arithmetic_expression)
}
pub fn simple_expression_simple_expression_string(
    _ctx: &Ctx,
    token_string_literal: TokenStringLiteral,
) -> SimpleExpression {
    write_to_symbol_table_file(&format!(
        "{}|{}|{}|{}",
        token_string_literal,
        "CONST_STRING",
        token_string_literal,
        token_string_literal.len()
    ));
    write_to_parser_file(&format!("<SimpleExpression> -> {token_string_literal}"));
    SimpleExpression::SimpleExpressionString(token_string_literal)
}
#[derive(Debug, Clone)]
pub enum Conjunction {
    ConjunctionAnd(TokenAnd),
    ConjunctionOr(TokenOr),
}
pub fn conjunction_conjunction_and(_ctx: &Ctx, token_and: TokenAnd) -> Conjunction {
    write_to_parser_file(&format!("<Conjunction> -> {token_and}"));
    Conjunction::ConjunctionAnd(token_and)
}
pub fn conjunction_conjunction_or(_ctx: &Ctx, token_or: TokenOr) -> Conjunction {
    write_to_parser_file(&format!("<Conjunction> -> {token_or}"));
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
pub fn comparison_op_comparison_op_equal(_ctx: &Ctx, token_equal: TokenEqual) -> ComparisonOp {
    write_to_parser_file(&format!("<ComparisonOp> -> {token_equal}"));
    ComparisonOp::ComparisonOpEqual(token_equal)
}
pub fn comparison_op_comparison_op_not_equal(
    _ctx: &Ctx,
    token_not_equal: TokenNotEqual,
) -> ComparisonOp {
    write_to_parser_file(&format!("<ComparisonOp> -> {token_not_equal}"));
    ComparisonOp::ComparisonOpNotEqual(token_not_equal)
}
pub fn comparison_op_comparison_op_less(_ctx: &Ctx, token_less: TokenLess) -> ComparisonOp {
    write_to_parser_file(&format!("<ComparisonOp> -> {token_less}"));
    ComparisonOp::ComparisonOpLess(token_less)
}
pub fn comparison_op_comparison_op_less_equal(
    _ctx: &Ctx,
    token_less_equal: TokenLessEqual,
) -> ComparisonOp {
    write_to_parser_file(&format!("<ComparisonOp> -> {token_less_equal}"));
    ComparisonOp::ComparisonOpLessEqual(token_less_equal)
}
pub fn comparison_op_comparison_op_greater(
    _ctx: &Ctx,
    token_greater: TokenGreater,
) -> ComparisonOp {
    write_to_parser_file(&format!("<ComparisonOp> -> {token_greater}"));
    ComparisonOp::ComparisonOpGreater(token_greater)
}
pub fn comparison_op_comparison_op_greater_equal(
    _ctx: &Ctx,
    token_greater_equal: TokenGreaterEqual,
) -> ComparisonOp {
    write_to_parser_file(&format!("<ComparisonOp> -> {token_greater_equal}"));
    ComparisonOp::ComparisonOpGreaterEqual(token_greater_equal)
}
#[derive(Debug, Clone)]
pub enum Number {
    NumberInt(TokenIntLiteral),
    NumberFloat(TokenFloatLiteral),
    NumberNegativeInt(NumberNegativeInt),
    NumberNegativeFloat(NumberNegativeFloat),
}
#[derive(Debug, Clone)]
pub struct NumberNegativeInt {
    pub token_sub: TokenSub,
    pub token_int_literal: TokenIntLiteral,
}
#[derive(Debug, Clone)]
pub struct NumberNegativeFloat {
    pub token_sub: TokenSub,
    pub token_float_literal: TokenFloatLiteral,
}
pub fn number_number_int(_ctx: &Ctx, token_int_literal: TokenIntLiteral) -> Number {
    write_to_symbol_table_file(&format!(
        "{}|{}|{}|{}",
        token_int_literal,
        "CONST_INT",
        token_int_literal,
        token_int_literal.to_string().len()
    ));
    write_to_parser_file(&format!("<Number> -> {token_int_literal}"));
    Number::NumberInt(token_int_literal)
}
pub fn number_number_float(_ctx: &Ctx, token_float_literal: TokenFloatLiteral) -> Number {
    write_to_symbol_table_file(&format!(
        "{}|{}|{}|{}",
        token_float_literal.original,
        "CONST_FLOAT",
        token_float_literal.original,
        token_float_literal.original.len()
    ));
    write_to_parser_file(&format!("<Number> -> {}", token_float_literal.original));
    Number::NumberFloat(token_float_literal)
}
pub fn number_number_negative_int(
    _ctx: &Ctx,
    token_sub: TokenSub,
    token_int_literal: TokenIntLiteral,
) -> Number {
    write_to_symbol_table_file(&format!(
        "-{}|{}|-{}|{}",
        token_int_literal,
        "CONST_INT",
        token_int_literal,
        token_int_literal.to_string().len() + 1
    ));
    write_to_parser_file(&format!("<Number> -> {token_sub} {token_int_literal}"));
    Number::NumberNegativeInt(NumberNegativeInt {
        token_sub,
        token_int_literal,
    })
}
pub fn number_number_negative_float(
    _ctx: &Ctx,
    token_sub: TokenSub,
    token_float_literal: TokenFloatLiteral,
) -> Number {
    write_to_symbol_table_file(&format!(
        "-{}|{}|-{}|{}",
        token_float_literal.original,
        "CONST_FLOAT",
        token_float_literal.original,
        token_float_literal.original.len() + 1
    ));
    write_to_parser_file(&format!(
        "<Number> -> {token_sub} {}",
        token_float_literal.original
    ));
    Number::NumberNegativeFloat(NumberNegativeFloat {
        token_sub,
        token_float_literal,
    })
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
    write_to_parser_file(&format!(
        "<NotStatement> -> {token_not} <BooleanExpression>"
    ));
    NotStatement {
        token_not,
        boolean_expression: Box::new(boolean_expression),
    }
}
#[derive(Debug, Clone)]
pub struct ArithmeticExpressionSumTerm {
    pub arithmetic_expression: Box<ArithmeticExpression>,
    pub token_sum: TokenSum,
    pub term: Term,
}
#[derive(Debug, Clone)]
pub struct ArithmeticExpressionSubTerm {
    pub arithmetic_expression: Box<ArithmeticExpression>,
    pub token_sub: TokenSub,
    pub term: Term,
}
#[derive(Debug, Clone)]
pub enum ArithmeticExpression {
    ArithmeticExpressionSumTerm(ArithmeticExpressionSumTerm),
    ArithmeticExpressionSubTerm(ArithmeticExpressionSubTerm),
    ArithmeticExpressionTerm(Term),
}
pub fn arithmetic_expression_arithmetic_expression_sum_term(
    _ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
    token_sum: TokenSum,
    term: Term,
) -> ArithmeticExpression {
    write_to_parser_file(&format!(
        "<ArithmeticExpression> -> <ArithmeticExpression> {token_sum} <Term>"
    ));
    ArithmeticExpression::ArithmeticExpressionSumTerm(ArithmeticExpressionSumTerm {
        arithmetic_expression: Box::new(arithmetic_expression),
        token_sum,
        term,
    })
}
pub fn arithmetic_expression_arithmetic_expression_sub_term(
    _ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
    token_sub: TokenSub,
    term: Term,
) -> ArithmeticExpression {
    write_to_parser_file(&format!(
        "<ArithmeticExpression> -> <ArithmeticExpression> {token_sub} <Term>"
    ));
    ArithmeticExpression::ArithmeticExpressionSubTerm(ArithmeticExpressionSubTerm {
        arithmetic_expression: Box::new(arithmetic_expression),
        token_sub,
        term,
    })
}
pub fn arithmetic_expression_arithmetic_expression_term(
    _ctx: &Ctx,
    term: Term,
) -> ArithmeticExpression {
    /// write_to_parser_file(&format!("<ArithmeticExpression> -> <Term>"));
    ArithmeticExpression::ArithmeticExpressionTerm(term)
}
#[derive(Debug, Clone)]
pub struct TermMulFactor {
    pub term: Box<Term>,
    pub token_mul: TokenMul,
    pub factor: Factor,
}
#[derive(Debug, Clone)]
pub struct TermDivFactor {
    pub term: Box<Term>,
    pub token_div: TokenDiv,
    pub factor: Factor,
}
#[derive(Debug, Clone)]
pub enum Term {
    TermMulFactor(TermMulFactor),
    TermDivFactor(TermDivFactor),
    TermFactor(Factor),
}
pub fn term_term_mul_factor(_ctx: &Ctx, term: Term, token_mul: TokenMul, factor: Factor) -> Term {
    write_to_parser_file(&format!("<Term> -> <Term> {token_mul} <Factor>"));
    Term::TermMulFactor(TermMulFactor {
        term: Box::new(term),
        token_mul,
        factor,
    })
}
pub fn term_term_div_factor(_ctx: &Ctx, term: Term, token_div: TokenDiv, factor: Factor) -> Term {
    write_to_parser_file(&format!("<Term> -> <Term> {token_div} <Factor>"));
    Term::TermDivFactor(TermDivFactor {
        term: Box::new(term),
        token_div,
        factor,
    })
}
pub fn term_term_factor(_ctx: &Ctx, factor: Factor) -> Term {
    /// write_to_parser_file(&format!("<Term> -> <Factor>"));
    Term::TermFactor(factor)
}
#[derive(Debug, Clone)]
pub struct FactorParen {
    pub token_par_open: TokenParOpen,
    pub arithmetic_expression: Box<ArithmeticExpression>,
    pub token_par_close: TokenParClose,
}
#[derive(Debug, Clone)]
pub enum Factor {
    FactorId(TokenId),
    FactorNumber(Number),
    FactorParen(FactorParen),
}
pub fn factor_factor_id(_ctx: &Ctx, token_id: TokenId) -> Factor {
    write_to_parser_file(&format!("<Factor> -> {token_id}"));
    Factor::FactorId(token_id)
}
pub fn factor_factor_number(_ctx: &Ctx, number: Number) -> Factor {
    write_to_parser_file("<Factor> -> <Number>");
    Factor::FactorNumber(number)
}
pub fn factor_factor_paren(
    _ctx: &Ctx,
    token_par_open: TokenParOpen,
    arithmetic_expression: ArithmeticExpression,
    token_par_close: TokenParClose,
) -> Factor {
    write_to_parser_file(&format!(
        "<Factor> -> {token_par_open} <ArithmeticExpression> {token_par_close}"
    ));
    Factor::FactorParen(FactorParen {
        token_par_open,
        arithmetic_expression: Box::new(arithmetic_expression),
        token_par_close,
    })
}
#[derive(Debug, Clone)]
pub enum IntegerValue {
    IntegerValueLiteral(TokenIntLiteral),
    IntegerValueId(TokenId),
}
pub fn integer_value_integer_value_literal(
    _ctx: &Ctx,
    token_int_literal: TokenIntLiteral,
) -> IntegerValue {
    write_to_symbol_table_file(&format!(
        "{}|{}|{}|{}",
        token_int_literal,
        "CONST_INT",
        token_int_literal,
        token_int_literal.to_string().len()
    ));
    write_to_parser_file(&format!("<IntegerValue> -> {token_int_literal}"));
    IntegerValue::IntegerValueLiteral(token_int_literal)
}
pub fn integer_value_integer_value_id(_ctx: &Ctx, token_id: TokenId) -> IntegerValue {
    write_to_parser_file(&format!("<IntegerValue> -> {token_id}"));
    IntegerValue::IntegerValueId(token_id)
}
impl VarDeclaration {
    pub fn write_to_symbol_table(&self) -> DataType {
        match self {
            Self::VarDeclarationSingle(single) => {
                write_to_symbol_table_file(&format!(
                    "{}|{}|{}|{}",
                    single.token_id,
                    single.data_type,
                    "-",
                    single.token_id.len()
                ));
                single.data_type.clone()
            }
            Self::VarDeclarationRecursive(recursive) => {
                let data_type = recursive.var_declaration.write_to_symbol_table();
                write_to_symbol_table_file(&format!(
                    "{}|{}|{}|{}",
                    recursive.token_id,
                    data_type,
                    "-",
                    recursive.token_id.len()
                ));
                data_type
            }
        }
    }
}
impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DataType::IntType(_) => "VAR_INT",
            DataType::FloatType(_) => "VAR_FLOAT",
            DataType::StringType(_) => "VAR_STRING",
        };
        write!(f, "{}", s)
    }
}
