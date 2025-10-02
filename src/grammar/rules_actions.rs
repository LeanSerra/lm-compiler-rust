/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use crate::compiler::context::{push_to_symbol_table, write_to_lexer_file, write_to_parser_file};
pub use crate::grammar::types::*;
use rustemo::Input;

/// Parses the keyword "int"
pub fn token_int(_ctx: &Ctx, token: Token) -> TokenInt {
    write_to_lexer_file(&format!("INT: {}", token.value));
    token.value.into()
}

/// Parses the keyword "float"
pub fn token_float(_ctx: &Ctx, token: Token) -> TokenFloat {
    write_to_lexer_file(&format!("FLOAT: {}", token.value));
    token.value.into()
}

/// Parses the keyword "string"
pub fn token_string(_ctx: &Ctx, token: Token) -> TokenString {
    write_to_lexer_file(&format!("STRING: {}", token.value));
    token.value.into()
}

/// Parses an integer literal into i64
///
/// # Safety
///
/// The parsing can't fail because we succesfully parse it in the lexer
pub fn token_int_literal(_ctx: &Ctx, token: Token) -> TokenIntLiteral {
    write_to_lexer_file(&format!("INT_LITERAL: {}", token.value));
    unsafe { token.value.parse().unwrap_unchecked() }
}

/// Parses a float literal into i64
///
/// # Safety
///
/// The parsing can't fail because we succesfully parse it in the lexer
pub fn token_float_literal(_ctx: &Ctx, token: Token) -> TokenFloatLiteral {
    write_to_lexer_file(&format!("FLOAT_LITERAL: {}", token.value));
    TokenFloatLiteral {
        original: token.value.to_string(),
        parsed: unsafe { token.value.parse::<f32>().unwrap_unchecked() },
    }
}

/// Parses a string literal by removing the "" and returning an owned string
pub fn token_string_literal(_ctx: &Ctx, mut token: Token) -> TokenStringLiteral {
    token.value = token.value.slice(1..token.value.len() - 1);
    write_to_lexer_file(&format!("STRING_LITERAL: {}", token.value));
    token.value.into()
}

/// Parses a TokenId
pub fn token_id(_ctx: &Ctx, token: Token) -> TokenId {
    write_to_lexer_file(&format!("ID: {}", token.value));
    token.value.into()
}

/// Parses the keyword ":="
pub fn token_assign(_ctx: &Ctx, token: Token) -> TokenAssign {
    write_to_lexer_file(&format!("ASSIGN: {}", token.value));
    token.value.into()
}

/// Parses the keyword "+"
pub fn token_sum(_ctx: &Ctx, token: Token) -> TokenSum {
    write_to_lexer_file(&format!("SUM: {}", token.value));
    token.value.into()
}

/// Parses the keyword "*"
pub fn token_mul(_ctx: &Ctx, token: Token) -> TokenMul {
    write_to_lexer_file(&format!("MUL: {}", token.value));
    token.value.into()
}

/// Parses the keyword "-"
pub fn token_sub(_ctx: &Ctx, token: Token) -> TokenSub {
    write_to_lexer_file(&format!("SUB: {}", token.value));
    token.value.into()
}

/// Parses the token "/"
pub fn token_div(_ctx: &Ctx, token: Token) -> TokenDiv {
    write_to_lexer_file(&format!("DIV: {}", token.value));
    token.value.into()
}

/// Parses the keyword "("
pub fn token_par_open(_ctx: &Ctx, token: Token) -> TokenParOpen {
    write_to_lexer_file(&format!("PAR_OPEN: {}", token.value));
    token.value.into()
}

/// Parses the keyword ")"
pub fn token_par_close(_ctx: &Ctx, token: Token) -> TokenParClose {
    write_to_lexer_file(&format!("PAR_CLOSE: {}", token.value));
    token.value.into()
}

/// Parses the keyword "{"
pub fn token_cbopen(_ctx: &Ctx, token: Token) -> TokenCBOpen {
    write_to_lexer_file(&format!("CB_OPEN: {}", token.value));
    token.value.into()
}

/// Parses the keyword "}"
pub fn token_cbclose(_ctx: &Ctx, token: Token) -> TokenCBClose {
    write_to_lexer_file(&format!("CB_CLOSE: {}", token.value));
    token.value.into()
}

/// Parses the keyword ":"
pub fn token_colon(_ctx: &Ctx, token: Token) -> TokenColon {
    write_to_lexer_file(&format!("COLON: {}", token.value));
    token.value.into()
}

/// Parses the keyword "init"
pub fn token_init(_ctx: &Ctx, token: Token) -> TokenInit {
    write_to_lexer_file(&format!("INIT: {}", token.value));
    token.value.into()
}

/// Parses the keyword "while"
pub fn token_while(_ctx: &Ctx, token: Token) -> TokenWhile {
    write_to_lexer_file(&format!("WHILE: {}", token.value));
    token.value.into()
}

/// Parses the keyword "=="
pub fn token_equal(_ctx: &Ctx, token: Token) -> TokenEqual {
    write_to_lexer_file(&format!("EQUAL: {}", token.value));
    token.value.into()
}

/// Parses the keyword "=="
pub fn token_not_equal(_ctx: &Ctx, token: Token) -> TokenNotEqual {
    write_to_lexer_file(&format!("NOT_EQUAL: {}", token.value));
    token.value.into()
}

/// Parses the keyword "<"
pub fn token_less(_ctx: &Ctx, token: Token) -> TokenLess {
    write_to_lexer_file(&format!("LESS: {}", token.value));
    token.value.into()
}

/// Parses the keyword "<="
pub fn token_less_equal(_ctx: &Ctx, token: Token) -> TokenLessEqual {
    write_to_lexer_file(&format!("LESS_EQUAL: {}", token.value));
    token.value.into()
}

/// Parses the keyword "<"
pub fn token_greater(_ctx: &Ctx, token: Token) -> TokenGreater {
    write_to_lexer_file(&format!("GREATER: {}", token.value));
    token.value.into()
}

/// Parses the keyword ">="
pub fn token_greater_equal(_ctx: &Ctx, token: Token) -> TokenGreaterEqual {
    write_to_lexer_file(&format!("GREATER_EQUAL: {}", token.value));
    token.value.into()
}

/// Parses the keyword "true"
pub fn token_true(_ctx: &Ctx, token: Token) -> TokenTrue {
    write_to_lexer_file(&format!("TRUE: {}", token.value));
    token.value.into()
}

/// Parses the keyword "false"
pub fn token_false(_ctx: &Ctx, token: Token) -> TokenFalse {
    write_to_lexer_file(&format!("FALSE: {}", token.value));
    token.value.into()
}

/// Parses the keyword "if"
pub fn token_if(_ctx: &Ctx, token: Token) -> TokenIf {
    write_to_lexer_file(&format!("IF: {}", token.value));
    token.value.into()
}

/// Parses the keyword "else"
pub fn token_else(_ctx: &Ctx, token: Token) -> TokenElse {
    write_to_lexer_file(&format!("ELSE: {}", token.value));
    token.value.into()
}

/// Parses the keyword ","
pub fn token_comma(_ctx: &Ctx, token: Token) -> TokenComma {
    write_to_lexer_file(&format!("COMMA: {}", token.value));
    token.value.into()
}

/// Pareses the keyword "and"
pub fn token_and(_ctx: &Ctx, token: Token) -> TokenAnd {
    write_to_lexer_file(&format!("AND: {}", token.value));
    token.value.into()
}

/// Parses the keyword "or"
pub fn token_or(_ctx: &Ctx, token: Token) -> TokenOr {
    write_to_lexer_file(&format!("OR: {}", token.value));
    token.value.into()
}

/// Parses the keyword "not"
pub fn token_not(_ctx: &Ctx, token: Token) -> TokenNot {
    write_to_lexer_file(&format!("NOT: {}", token.value));
    token.value.into()
}

/// Parses the keyword "read"
pub fn token_read(_ctx: &Ctx, token: Token) -> TokenRead {
    write_to_lexer_file(&format!("READ: {}", token.value));
    token.value.into()
}

/// Parses the keyword "write"
pub fn token_write(_ctx: &Ctx, token: Token) -> TokenWrite {
    write_to_lexer_file(&format!("WRITE: {}", token.value));
    token.value.into()
}

/// Parses the keyword "isZero"
pub fn token_is_zero(_ctx: &Ctx, token: Token) -> TokenIsZero {
    write_to_lexer_file(&format!("IS_ZERO: {}", token.value));
    token.value.into()
}

/// Parses the keyword "convDate"
pub fn token_conv_date(_ctx: &Ctx, token: Token) -> TokenConvDate {
    write_to_lexer_file(&format!("CONV_DATE: {}", token.value));
    token.value.into()
}

/// Parses a date in “DD-MM-YYYY” format
pub fn token_date(_ctx: &Ctx, token: Token) -> TokenDate {
    write_to_lexer_file(&format!("DATE: {}", token.value));
    TokenDate {
        day: token.value.get(0..2).unwrap().into(),
        month: token.value.get(3..5).unwrap().into(),
        year: token.value.get(6..).unwrap().into(),
    }
}

/// Parses the rule `<Program> -> TokenId TokenParOpen TokenParClose TokenCBOpen <Body> TokenCBClose`
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

/// Parses the rule `<Program> -> <Body>`
pub fn program_program_only_body(_ctx: &Ctx, body: Body) -> Program {
    write_to_parser_file("<Program> -> <Body>");
    Program::ProgramOnlyBody(body)
}

/// Parses the rule `<Body> -> TokenInit <InitBody> <Expressions>`
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

/// Parses the rule `<Body> -> TokenInit <InitBody>`
pub fn body_body_init(_ctx: &Ctx, token_init: TokenInit, init_body: InitBody) -> Body {
    write_to_parser_file(&format!("<Body> -> {token_init} <InitBody>"));
    Some(BodyNoO::BodyInit(BodyInit {
        token_init,
        init_body,
    }))
}

/// Parses the rule `<Body> -> <Expressions>`
pub fn body_body_expressions(_ctx: &Ctx, expressions: Expressions) -> Body {
    write_to_parser_file("<Body> -> <Expressions>");
    Some(BodyNoO::BodyExpressions(expressions))
}

/// Parses the rule `<Body> -> EMPTY`
pub fn body_body_empty(_ctx: &Ctx) -> Body {
    write_to_parser_file("<Body> -> EMPTY");
    None
}

/// Parses the rule `<Body> -> TokenCBOpen <VarDeclarations> TokenCBClose`
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

/// Parses the rule `<FunctionRead> -> TokenRead TokenParOpen TokenId TokenParClose`
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

/// Parses the rule `<FunctionWrite> -> TokenWrite TokenParOpen <SimpleExpression> TokenParClose`
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

/// Parses the rule `<FunctionIsZero>: TokenIsZero TokenParOpen <ArithmeticExpression> TokenParClose`
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

/// Parses the rule `<FunctionConvDate>: TokenConvDate TokenParOpen TokenDate TokenParClose`
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

/// Parses the rule `<VarDeclarations> -> <VarDeclaration>`
pub fn var_declarations_var_declarations_single(
    _ctx: &Ctx,
    var_declaration: VarDeclaration,
) -> VarDeclarations {
    var_declaration.push_to_symbol_table();
    write_to_parser_file("<VarDeclarations> -> <VarDeclaration>");
    VarDeclarations::VarDeclarationsSingle(var_declaration)
}

/// Parses the `<VarDeclarations> -> <VarDeclaration> <VarDeclarations>`
pub fn var_declarations_var_declarations_recursive(
    _ctx: &Ctx,
    var_declaration: VarDeclaration,
    var_declarations: VarDeclarations,
) -> VarDeclarations {
    var_declaration.push_to_symbol_table();
    write_to_parser_file("<VarDeclarations> -> <VarDeclaration> <VarDeclarations>");
    VarDeclarations::VarDeclarationsRecursive(VarDeclarationsRecursive {
        var_declaration,
        var_declarations: Box::new(var_declarations),
    })
}

/// Parses the rule `<VarDeclaration> -> TokenId TokenColon <DataType>`
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

/// Parses the rule `<VarDeclaration> -> TokenId TokenComma <VarDeclaration>`
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

/// Parses the rule `<Expressions> -> <Statement>`
pub fn expressions_expression_single(_ctx: &Ctx, statement: Statement) -> Expressions {
    write_to_parser_file("<Expressions> -> <Statement>");
    Expressions::ExpressionSingle(statement)
}

/// Parses the rule `<Expressions> -> <Statement> <Expressions>`
pub fn expressions_expression_recursive(
    _ctx: &Ctx,
    statement: Statement,
    expressions: Expressions,
) -> Expressions {
    write_to_parser_file("<Expressions> -> <Statement> <Expressions>");
    Expressions::ExpressionRecursive(ExpressionRecursive {
        statement,
        expressions: Box::new(expressions),
    })
}

/// Parses the rule `<Statement> -> <Assignment>`
pub fn statement_statement_assignment(_ctx: &Ctx, assignment: Assignment) -> Statement {
    write_to_parser_file("<Statement> -> <Assignment>");
    Statement::StatementAssignment(assignment)
}

/// Parses the rule `<Statement> -> <IfStatement>`
pub fn statement_statement_if_statement(_ctx: &Ctx, if_statement: IfStatement) -> Statement {
    write_to_parser_file("<Statement> -> <IfStatement>");
    Statement::StatementIfStatement(if_statement)
}

/// Parses the rule `<Statement> -> <ElseStatement>`
pub fn statement_statement_else_statement(_ctx: &Ctx, else_statement: ElseStatement) -> Statement {
    write_to_parser_file("<Statement> -> <ElseStatement>");
    Statement::StatementElseStatement(else_statement)
}

/// Parses the rule `<Statement> -> <WhileLoop>`
pub fn statement_statement_while(_ctx: &Ctx, while_loop: WhileLoop) -> Statement {
    write_to_parser_file("<Statement> -> <WhileLoop>");
    Statement::StatementWhile(while_loop)
}

/// Parses the rule `<Statement> -> <FunctionWrite>`
pub fn statement_statement_write(_ctx: &Ctx, function_write: FunctionWrite) -> Statement {
    write_to_parser_file("<Statement> -> <FunctionWrite>");
    Statement::StatementWrite(function_write)
}

/// Parses the rule `<Statement> -> <FunctionRead>`
pub fn statement_statement_read(_ctx: &Ctx, function_read: FunctionRead) -> Statement {
    write_to_parser_file("<Statement> -> <FunctionRead>");
    Statement::StatementRead(function_read)
}

/// Parses the rule `<Assignment> -> TokenId TokenAssign <SimpleExpression>`
pub fn assignment_assignment_expression(
    _ctx: &Ctx,
    token_id: TokenId,
    token_assign: TokenAssign,
    simple_expression: SimpleExpression,
) -> Assignment {
    write_to_parser_file(&format!(
        "<Assignment> -> {token_id} {token_assign} <SimpleExpression>"
    ));
    Assignment::AssignmentExpression(AssignmentExpression {
        token_id,
        token_assign,
        simple_expression,
    })
}

/// Parses the rule `<Assignment> -> TokenId TokenAssign <FunctionConvDate>`
pub fn assignment_assignment_conv_date(
    _ctx: &Ctx,
    token_id: TokenId,
    token_assign: TokenAssign,
    function_conv_date: FunctionConvDate,
) -> Assignment {
    write_to_parser_file(&format!(
        "<Assignment> -> {token_id} {token_assign} <FunctionConvDate>"
    ));
    Assignment::AssignmentConvDate(ConvDate {
        token_id,
        token_assign,
        function_conv_date,
    })
}

/// Parses the rule `<DataType> -> "int"`
pub fn data_type_int_type(_ctx: &Ctx, token_int: TokenInt) -> DataType {
    write_to_parser_file(&format!("<DataType> -> {token_int}"));
    DataType::IntType(token_int)
}

/// Parses the rule `<DataType> -> "float"`
pub fn data_type_float_type(_ctx: &Ctx, token_float: TokenFloat) -> DataType {
    write_to_parser_file(&format!("<DataType> -> {token_float}"));
    DataType::FloatType(token_float)
}

/// Parses the rule `<DataType> -> "string"`
pub fn data_type_string_type(_ctx: &Ctx, token_string: TokenString) -> DataType {
    write_to_parser_file(&format!("<DataType> -> {token_string}"));
    DataType::StringType(token_string)
}

/// Parses the rule `<WhileLoop> -> TokenWhile TokenParOpen <BooleanExpression> TokenParClose TokenCBOpen <Body> TokenCBClose`
#[expect(clippy::too_many_arguments)]
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

/// Parses the rule `<IfStatement>: TokenIf TokenParOpen <BooleanExpression> TokenParClose TokenCBOpen <Body> TokenCBClose`
#[expect(clippy::too_many_arguments)]
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

/// Parses the rule `<ElseStatement>: TokenElse TokenCBOpen <Body> TokenCBClose`
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

/// Parses the rule `<BooleanExpression> -> <SimpleExpression> <BooleanExpressionChain>`
pub fn boolean_expression_boolean_expression_simple_expression(
    _ctx: &Ctx,
    simple_expression: SimpleExpression,
    boolean_expression_chain: BooleanExpressionChain,
) -> BooleanExpression {
    write_to_parser_file("<BooleanExpression> -> <SimpleExpression> <BooleanExpressionChain>");
    BooleanExpression::BooleanExpressionSimpleExpression(BooleanExpressionSimpleExpression {
        simple_expression,
        boolean_expression_chain,
    })
}

/// Parses the rule `<BooleanExpression> -> "true"`
pub fn boolean_expression_boolean_expression_true(
    _ctx: &Ctx,
    token_true: TokenTrue,
) -> BooleanExpression {
    write_to_parser_file(&format!("<BooleanExpression> -> {token_true}"));
    BooleanExpression::BooleanExpressionTrue(token_true)
}

/// Parses the rule `<BooleanExpression> -> "false"`
pub fn boolean_expression_boolean_expression_false(
    _ctx: &Ctx,
    token_false: TokenFalse,
) -> BooleanExpression {
    write_to_parser_file(&format!("<BooleanExpression> -> {token_false}"));
    BooleanExpression::BooleanExpressionFalse(token_false)
}

/// Parses the rule `<BooleanExpression> -> <SimpleExpression> <BooleanExpressionChain> <Conjunction> <BooleanExpression>`
pub fn boolean_expression_boolean_expression_simple_expression_recursive(
    _ctx: &Ctx,
    simple_expression: SimpleExpression,
    boolean_expression_chain: BooleanExpressionChain,
    conjunction: Conjunction,
    boolean_expression: BooleanExpression,
) -> BooleanExpression {
    write_to_parser_file(
        "<BooleanExpression> -> <SimpleExpression> <BooleanExpressionChain> <Conjunction> <BooleanExpression>",
    );
    BooleanExpression::BooleanExpressionSimpleExpressionRecursive(
        BooleanExpressionSimpleExpressionRecursive {
            simple_expression,
            boolean_expression_chain,
            conjunction,
            boolean_expression: Box::new(boolean_expression),
        },
    )
}

/// Parses the rule `<BooleanExpression> -> <NotStatement>`
pub fn boolean_expression_boolean_expression_not_statement(
    _ctx: &Ctx,
    not_statement: NotStatement,
) -> BooleanExpression {
    write_to_parser_file("<BooleanExpression> -> <NotStatement>");
    BooleanExpression::BooleanExpressionNotStatement(not_statement)
}

/// Parses thre rule `<BooleanExpression> -> <FunctionIsZero>`
pub fn boolean_expression_boolean_expression_is_zero(
    _ctx: &Ctx,
    function_is_zero: FunctionIsZero,
) -> BooleanExpression {
    write_to_parser_file("<BooleanExpression> -> <FunctionIsZero>");
    BooleanExpression::BooleanExpressionIsZero(function_is_zero)
}

/// Parses the rule `<BooleanExpressionChain> -> ComparisonOp <SimpleExpression> <BooleanExpressionChain>`
pub fn boolean_expression_chain_boolean_expression_chain_aux(
    _ctx: &Ctx,
    comparison_op: ComparisonOp,
    simple_expression: SimpleExpression,
    boolean_expression_chain: BooleanExpressionChain,
) -> BooleanExpressionChain {
    write_to_parser_file(
        "<BooleanExpressionChain> -> <ComparisonOp> <SimpleExpression> <BooleanExpressionChain>",
    );
    Some(BooleanExpressionChainNoO {
        comparison_op,
        simple_expression,
        boolean_expression_chain: Box::new(boolean_expression_chain),
    })
}

/// Parses the rule `<BooleanExpressionChain> -> EMPTY`
pub fn boolean_expression_chain_boolean_expression_chain_empty(
    _ctx: &Ctx,
) -> BooleanExpressionChain {
    write_to_parser_file("<BooleanExpressionChain> -> EMPTY");
    None
}

/// Parses the rule `<SimpleExpression> -> <ArithmeticExpression>`
pub fn simple_expression_simple_expression_arithmetic(
    _ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
) -> SimpleExpression {
    write_to_parser_file("<SimpleExpression> -> <ArithmeticExpression>");
    SimpleExpression::SimpleExpressionArithmeticExpression(arithmetic_expression)
}

/// Parses the rule `<SimpleExpression> -> TokenStringLiteral`
pub fn simple_expression_simple_expression_string(
    _ctx: &Ctx,
    token_string_literal: TokenStringLiteral,
) -> SimpleExpression {
    push_to_symbol_table(token_string_literal.clone().into());
    write_to_parser_file(&format!("<SimpleExpression> -> {token_string_literal}"));
    SimpleExpression::SimpleExpressionString(token_string_literal)
}

/// Parses the rule `<Conjunction> -> "and"`
pub fn conjunction_conjunction_and(_ctx: &Ctx, token_and: TokenAnd) -> Conjunction {
    write_to_parser_file(&format!("<Conjunction> -> {token_and}"));
    Conjunction::ConjunctionAnd(token_and)
}

/// Parses the rule `<Conjunction> -> "or"`
pub fn conjunction_conjunction_or(_ctx: &Ctx, token_or: TokenOr) -> Conjunction {
    write_to_parser_file(&format!("<Conjunction> -> {token_or}"));
    Conjunction::ConjunctionOr(token_or)
}

/// Parses the rule `<ComparisonOp> -> "=="`
pub fn comparison_op_comparison_op_equal(_ctx: &Ctx, token_equal: TokenEqual) -> ComparisonOp {
    write_to_parser_file(&format!("<ComparisonOp> -> {token_equal}"));
    ComparisonOp::ComparisonOpEqual(token_equal)
}

/// Parses the rule  `<ComparisonOp> -> "!="`
pub fn comparison_op_comparison_op_not_equal(
    _ctx: &Ctx,
    token_not_equal: TokenNotEqual,
) -> ComparisonOp {
    write_to_parser_file(&format!("<ComparisonOp> -> {token_not_equal}"));
    ComparisonOp::ComparisonOpNotEqual(token_not_equal)
}

/// Parses the rule `<ComparisonOp> -> "<"`
pub fn comparison_op_comparison_op_less(_ctx: &Ctx, token_less: TokenLess) -> ComparisonOp {
    write_to_parser_file(&format!("<ComparisonOp> -> {token_less}"));
    ComparisonOp::ComparisonOpLess(token_less)
}

/// Parses the rule `<ComparisonOp> -> ">="`
pub fn comparison_op_comparison_op_less_equal(
    _ctx: &Ctx,
    token_less_equal: TokenLessEqual,
) -> ComparisonOp {
    write_to_parser_file(&format!("<ComparisonOp> -> {token_less_equal}"));
    ComparisonOp::ComparisonOpLessEqual(token_less_equal)
}

/// Parses the rule `<ComparisonOp> -> ">"`
pub fn comparison_op_comparison_op_greater(
    _ctx: &Ctx,
    token_greater: TokenGreater,
) -> ComparisonOp {
    write_to_parser_file(&format!("<ComparisonOp> -> {token_greater}"));
    ComparisonOp::ComparisonOpGreater(token_greater)
}

/// Parses the rule `<ComparisonOp> -> ">="`
pub fn comparison_op_comparison_op_greater_equal(
    _ctx: &Ctx,
    token_greater_equal: TokenGreaterEqual,
) -> ComparisonOp {
    write_to_parser_file(&format!("<ComparisonOp> -> {token_greater_equal}"));
    ComparisonOp::ComparisonOpGreaterEqual(token_greater_equal)
}

/// Parses the rule `<Number> -> TokenIntLiteral`
pub fn number_number_int(_ctx: &Ctx, token_int_literal: TokenIntLiteral) -> Number {
    push_to_symbol_table(token_int_literal.into());
    write_to_parser_file(&format!("<Number> -> {token_int_literal}"));
    Number::NumberInt(token_int_literal)
}

/// Parses the rule `<Number> -> TokenFloatLiteral`
pub fn number_number_float(_ctx: &Ctx, token_float_literal: TokenFloatLiteral) -> Number {
    push_to_symbol_table(token_float_literal.clone().into());
    write_to_parser_file(&format!("<Number> -> {}", token_float_literal.original));
    Number::NumberFloat(token_float_literal)
}

/// Parses the rule `<Number> -> TokenSub TokenIntLiteral`
pub fn number_number_negative_int(
    _ctx: &Ctx,
    token_sub: TokenSub,
    token_int_literal: TokenIntLiteral,
) -> Number {
    let value: i64 = unsafe {
        format!("{token_sub}{token_int_literal}")
            .parse()
            .unwrap_unchecked()
    };
    push_to_symbol_table(value.into());
    write_to_parser_file(&format!("<Number> -> {token_sub} {token_int_literal}"));
    Number::NumberInt(value)
}

/// Parses the rule `<Number> -> TokenSub TokenFloatLiteral`
pub fn number_number_negative_float(
    _ctx: &Ctx,
    token_sub: TokenSub,
    mut token_float_literal: TokenFloatLiteral,
) -> Number {
    token_float_literal.original = format!("{token_sub}{}", token_float_literal.original);
    token_float_literal.parsed *= -1_f32;
    push_to_symbol_table(token_float_literal.clone().into());
    write_to_parser_file(&format!(
        "<Number> -> {token_sub} {}",
        token_float_literal.original
    ));
    Number::NumberFloat(token_float_literal)
}

/// Parses the rule `<NotStatement> -> TokenNot <BooleanExpression>`
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

/// Parses the rule `<ArithmeticExpression> -> <ArithmeticExpression> TokenSum <Term>`
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

/// Parses the rule `<ArithmeticExpression> -> <ArithmeticExpression> TokenSub <Term>`
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

/// Parses the rule `<ArithmeticExpression> -> <Term>`
pub fn arithmetic_expression_arithmetic_expression_term(
    _ctx: &Ctx,
    term: Term,
) -> ArithmeticExpression {
    write_to_parser_file("<ArithmeticExpression> -> <Term>");
    ArithmeticExpression::ArithmeticExpressionTerm(term)
}

/// Parses the rule `<Term> -> <Term> TokenMul <Factor>`
pub fn term_term_mul_factor(_ctx: &Ctx, term: Term, token_mul: TokenMul, factor: Factor) -> Term {
    write_to_parser_file(&format!("<Term> -> <Term> {token_mul} <Factor>"));
    Term::TermMulFactor(TermMulFactor {
        term: Box::new(term),
        token_mul,
        factor,
    })
}

/// Parses the rule `<Term> -> <Term> TokenDiv <Factor>`
pub fn term_term_div_factor(_ctx: &Ctx, term: Term, token_div: TokenDiv, factor: Factor) -> Term {
    write_to_parser_file(&format!("<Term> -> <Term> {token_div} <Factor>"));
    Term::TermDivFactor(TermDivFactor {
        term: Box::new(term),
        token_div,
        factor,
    })
}

/// Parses the rule `<Term> -> <Factor>`
pub fn term_term_factor(_ctx: &Ctx, factor: Factor) -> Term {
    write_to_parser_file("<Term> -> <Factor>");
    Term::TermFactor(factor)
}

/// Parses the rule `<Factor> -> TokenId`
pub fn factor_factor_id(_ctx: &Ctx, token_id: TokenId) -> Factor {
    write_to_parser_file(&format!("<Factor> -> {token_id}"));
    Factor::FactorId(token_id)
}

/// Parses the rule `<Factor> -> <Number>`
pub fn factor_factor_number(_ctx: &Ctx, number: Number) -> Factor {
    write_to_parser_file("<Factor> -> <Number>");
    Factor::FactorNumber(number)
}

/// Parses the rule `<Factor> -> TokenParOpen <ArithmeticExpression> TokenParClose`
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
