use crate::compiler::{
    ast::{AstAction, AstPtr, Node, NodeValue},
    context::CompilerContext,
    error::{CompilerError, log_error_and_exit},
};
pub use crate::grammar::types::*;
use rustemo::{Context, Input};
use std::rc::Rc;

/// Parses the keyword "int"
pub fn token_int(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenInt {
    compiler_context.write_to_lexer_file(&format!("INT: {}", token.value));
    token.value.into()
}

/// Parses the keyword "float"
pub fn token_float(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenFloat {
    compiler_context.write_to_lexer_file(&format!("FLOAT: {}", token.value));
    token.value.into()
}

/// Parses the keyword "string"
pub fn token_string(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenString {
    compiler_context.write_to_lexer_file(&format!("STRING: {}", token.value));
    token.value.into()
}

/// Parses an integer literal into i64
///
/// # Safety
///
/// The parsing can't fail because we succesfully parse it in the lexer
pub fn token_int_literal(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenIntLiteral {
    compiler_context.write_to_lexer_file(&format!("INT_LITERAL: {}", token.value));
    token.value.parse().unwrap()
}

/// Parses a float literal into i64
///
/// # Safety
///
/// The parsing can't fail because we succesfully parse it in the lexer
pub fn token_float_literal(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenFloatLiteral {
    compiler_context.write_to_lexer_file(&format!("FLOAT_LITERAL: {}", token.value));
    TokenFloatLiteral {
        original: token.value.to_string(),
        parsed: token.value.parse::<f32>().unwrap(),
    }
}

/// Parses a string literal by removing the "" and returning an owned string
pub fn token_string_literal(
    _ctx: &Ctx,
    mut token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenStringLiteral {
    token.value = token.value.slice(1..token.value.len() - 1);
    compiler_context.write_to_lexer_file(&format!("STRING_LITERAL: {}", token.value));
    token.value.into()
}

/// Parses a TokenId
pub fn token_id(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenId {
    compiler_context.write_to_lexer_file(&format!("ID: {}", token.value));
    token.value.into()
}

/// Parses the keyword ":="
pub fn token_assign(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenAssign {
    compiler_context.write_to_lexer_file(&format!("ASSIGN: {}", token.value));
    token.value.into()
}

/// Parses the keyword "+"
pub fn token_sum(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenSum {
    compiler_context.write_to_lexer_file(&format!("SUM: {}", token.value));
    token.value.into()
}

/// Parses the keyword "*"
pub fn token_mul(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenMul {
    compiler_context.write_to_lexer_file(&format!("MUL: {}", token.value));
    token.value.into()
}

/// Parses the keyword "-"
pub fn token_sub(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenSub {
    compiler_context.write_to_lexer_file(&format!("SUB: {}", token.value));
    token.value.into()
}

/// Parses the token "/"
pub fn token_div(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenDiv {
    compiler_context.write_to_lexer_file(&format!("DIV: {}", token.value));
    token.value.into()
}

/// Parses the keyword "("
pub fn token_par_open(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenParOpen {
    compiler_context.write_to_lexer_file(&format!("PAR_OPEN: {}", token.value));
    token.value.into()
}

/// Parses the keyword ")"
pub fn token_par_close(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenParClose {
    compiler_context.write_to_lexer_file(&format!("PAR_CLOSE: {}", token.value));
    token.value.into()
}

/// Parses the keyword "{"
pub fn token_cbopen(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenCBOpen {
    compiler_context.write_to_lexer_file(&format!("CB_OPEN: {}", token.value));
    token.value.into()
}

/// Parses the keyword "}"
pub fn token_cbclose(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenCBClose {
    compiler_context.write_to_lexer_file(&format!("CB_CLOSE: {}", token.value));
    token.value.into()
}

/// Parses the keyword ":"
pub fn token_colon(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenColon {
    compiler_context.write_to_lexer_file(&format!("COLON: {}", token.value));
    token.value.into()
}

/// Parses the keyword "init"
pub fn token_init(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenInit {
    compiler_context.write_to_lexer_file(&format!("INIT: {}", token.value));
    token.value.into()
}

/// Parses the keyword "while"
pub fn token_while(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenWhile {
    compiler_context.write_to_lexer_file(&format!("WHILE: {}", token.value));
    token.value.into()
}

/// Parses the keyword "=="
pub fn token_equal(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenEqual {
    compiler_context.write_to_lexer_file(&format!("EQUAL: {}", token.value));
    token.value.into()
}

/// Parses the keyword "=="
pub fn token_not_equal(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenNotEqual {
    compiler_context.write_to_lexer_file(&format!("NOT_EQUAL: {}", token.value));
    token.value.into()
}

/// Parses the keyword "<"
pub fn token_less(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenLess {
    compiler_context.write_to_lexer_file(&format!("LESS: {}", token.value));
    token.value.into()
}

/// Parses the keyword "<="
pub fn token_less_equal(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenLessEqual {
    compiler_context.write_to_lexer_file(&format!("LESS_EQUAL: {}", token.value));
    token.value.into()
}

/// Parses the keyword "<"
pub fn token_greater(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenGreater {
    compiler_context.write_to_lexer_file(&format!("GREATER: {}", token.value));
    token.value.into()
}

/// Parses the keyword ">="
pub fn token_greater_equal(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenGreaterEqual {
    compiler_context.write_to_lexer_file(&format!("GREATER_EQUAL: {}", token.value));
    token.value.into()
}

/// Parses the keyword "true"
pub fn token_true(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenTrue {
    compiler_context.write_to_lexer_file(&format!("TRUE: {}", token.value));
    token.value.into()
}

/// Parses the keyword "false"
pub fn token_false(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenFalse {
    compiler_context.write_to_lexer_file(&format!("FALSE: {}", token.value));
    token.value.into()
}

/// Parses the keyword "if"
pub fn token_if(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenIf {
    compiler_context.write_to_lexer_file(&format!("IF: {}", token.value));
    token.value.into()
}

/// Parses the keyword "else"
pub fn token_else(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenElse {
    compiler_context.write_to_lexer_file(&format!("ELSE: {}", token.value));
    token.value.into()
}

/// Parses the keyword ","
pub fn token_comma(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenComma {
    compiler_context.write_to_lexer_file(&format!("COMMA: {}", token.value));
    token.value.into()
}

/// Pareses the keyword "and"
pub fn token_and(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenAnd {
    compiler_context.write_to_lexer_file(&format!("AND: {}", token.value));
    token.value.into()
}

/// Parses the keyword "or"
pub fn token_or(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenOr {
    compiler_context.write_to_lexer_file(&format!("OR: {}", token.value));
    token.value.into()
}

/// Parses the keyword "not"
pub fn token_not(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenNot {
    compiler_context.write_to_lexer_file(&format!("NOT: {}", token.value));
    token.value.into()
}

/// Parses the keyword "read"
pub fn token_read(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenRead {
    compiler_context.write_to_lexer_file(&format!("READ: {}", token.value));
    token.value.into()
}

/// Parses the keyword "write"
pub fn token_write(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenWrite {
    compiler_context.write_to_lexer_file(&format!("WRITE: {}", token.value));
    token.value.into()
}

/// Parses the keyword "isZero"
pub fn token_is_zero(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenIsZero {
    compiler_context.write_to_lexer_file(&format!("IS_ZERO: {}", token.value));
    token.value.into()
}

/// Parses the keyword "convDate"
pub fn token_conv_date(
    _ctx: &Ctx,
    token: Token,
    compiler_context: &mut CompilerContext,
) -> TokenConvDate {
    compiler_context.write_to_lexer_file(&format!("CONV_DATE: {}", token.value));
    token.value.into()
}

/// Parses a date in “DD-MM-YYYY” format
pub fn token_date(_ctx: &Ctx, token: Token, compiler_context: &mut CompilerContext) -> TokenDate {
    compiler_context.write_to_lexer_file(&format!("DATE: {}", token.value));
    TokenDate {
        day: token.value.get(0..2).unwrap().into(),
        month: token.value.get(3..5).unwrap().into(),
        year: token.value.get(6..).unwrap().into(),
    }
}

/// Parses the rule `<Program> -> TokenId TokenParOpen TokenParClose TokenCBOpen <Body> TokenCBClose`
#[expect(clippy::too_many_arguments)]
pub fn program_program_with_main(
    _ctx: &Ctx,
    token_id: TokenId,
    token_par_open: TokenParOpen,
    token_par_close: TokenParClose,
    token_cbopen: TokenCBOpen,
    body: Body,
    token_cbclose: TokenCBClose,
    compiler_context: &mut CompilerContext,
) -> Program {
    compiler_context.write_to_parser_file(&format!(
        "<Program> -> {token_id} {token_par_open} {token_par_close} {token_cbopen} <Body> {token_cbclose}"
    ));
    compiler_context
        .ast
        .assign_node_to_ptr(AstPtr::Body.into(), AstPtr::Program);
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
pub fn program_program_only_body(
    _ctx: &Ctx,
    body: Body,
    compiler_context: &mut CompilerContext,
) -> Program {
    compiler_context.write_to_parser_file("<Program> -> <Body>");
    compiler_context
        .ast
        .assign_node_to_ptr(AstPtr::Body.into(), AstPtr::Program);
    Program::ProgramOnlyBody(body)
}

/// Parses the rule `<Body> -> TokenInit <InitBody> <Expressions>`
pub fn body_body_init_expressions(
    _ctx: &Ctx,
    token_init: TokenInit,
    init_body: InitBody,
    expressions: Expressions,
    compiler_context: &mut CompilerContext,
) -> Body {
    compiler_context
        .write_to_parser_file(&format!("<Body> -> {token_init} <InitBody> <Expressions>"));
    compiler_context
        .ast
        .assign_node_to_ptr(AstPtr::Expressions.into(), AstPtr::Body);
    Some(BodyNoO::BodyInitExpressions(BodyInitExpressions {
        token_init,
        init_body,
        expressions,
    }))
}

/// Parses the rule `<Body> -> TokenInit <InitBody>`
pub fn body_body_init(
    _ctx: &Ctx,
    token_init: TokenInit,
    init_body: InitBody,
    compiler_context: &mut CompilerContext,
) -> Body {
    compiler_context.write_to_parser_file(&format!("<Body> -> {token_init} <InitBody>"));
    Some(BodyNoO::BodyInit(BodyInit {
        token_init,
        init_body,
    }))
}

/// Parses the rule `<Body> -> <Expressions>`
pub fn body_body_expressions(
    _ctx: &Ctx,
    expressions: Expressions,
    compiler_context: &mut CompilerContext,
) -> Body {
    compiler_context.write_to_parser_file("<Body> -> <Expressions>");
    compiler_context
        .ast
        .assign_node_to_ptr(AstPtr::Expressions.into(), AstPtr::Body);
    Some(BodyNoO::BodyExpressions(expressions))
}

/// Parses the rule `<Body> -> EMPTY`
pub fn body_body_empty(_ctx: &Ctx, compiler_context: &mut CompilerContext) -> Body {
    compiler_context.write_to_parser_file("<Body> -> EMPTY");
    let leaf = Node::new_leaf(NodeValue::Action(AstAction::Noop));
    compiler_context
        .ast
        .assign_node_to_ptr(Rc::new(leaf).into(), AstPtr::Body);
    None
}

/// Parses the rule `<Body> -> TokenCBOpen <VarDeclarations> TokenCBClose`
pub fn init_body_init_body(
    _ctx: &Ctx,
    token_cbopen: TokenCBOpen,
    var_declarations: VarDeclarations,
    token_cbclose: TokenCBClose,
    compiler_context: &mut CompilerContext,
) -> InitBody {
    compiler_context.write_to_parser_file(&format!(
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
    compiler_context: &mut CompilerContext,
) -> FunctionRead {
    compiler_context.write_to_parser_file(&format!(
        "<FunctionRead> -> {token_read} {token_par_open} {token_id} {token_par_close}"
    ));
    let right_child = Node::new_leaf(NodeValue::Action(AstAction::Noop));
    let left_child = Node::new_leaf(NodeValue::Value(token_id.clone()));
    compiler_context.ast.create_node(
        AstAction::Read,
        Rc::new(left_child).into(),
        Rc::new(right_child).into(),
        AstPtr::Read,
    );
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
    compiler_context: &mut CompilerContext,
) -> FunctionWrite {
    compiler_context.write_to_parser_file(&format!(
        "<FunctionWrite> -> {token_write} {token_par_open} <SimpleExpression> {token_par_close}"
    ));
    let leaf = Node::new_leaf(NodeValue::Action(AstAction::Noop));
    compiler_context.ast.create_node(
        AstAction::Write,
        AstPtr::SimpleExpression.into(),
        Rc::new(leaf).into(),
        AstPtr::Write,
    );
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
    compiler_context: &mut CompilerContext,
) -> FunctionIsZero {
    compiler_context.write_to_parser_file(&format!(
        "<FunctionIsZero> -> {token_is_zero} {token_par_open} <E> {token_par_close}"
    ));
    let zero_leaf = Rc::new(Node::new_leaf(NodeValue::Value("0".into())));
    compiler_context.ast.create_node(
        AstAction::EQ,
        AstPtr::ArithmeticExpression.into(),
        zero_leaf.into(),
        AstPtr::IsZero,
    );
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
    compiler_context: &mut CompilerContext,
) -> FunctionConvDate {
    compiler_context.write_to_parser_file(&format!(
        "<FunctionConvDate> -> {token_conv_date} {token_par_open} {token_date} {token_par_close}"
    ));
    let thousand_leaf = Rc::new(Node::new_leaf(NodeValue::Value("1000".into())));
    let hundread_leaf = Rc::new(Node::new_leaf(NodeValue::Value("100".into())));
    let one_leaf = Rc::new(Node::new_leaf(NodeValue::Value("1".into())));

    let year_leaf = Rc::new(Node::new_leaf(NodeValue::Value(token_date.year.clone())));
    let month_leaf = Rc::new(Node::new_leaf(NodeValue::Value(token_date.month.clone())));
    let day_leaf = Rc::new(Node::new_leaf(NodeValue::Value(token_date.day.clone())));

    let year_node = compiler_context.ast.create_node(
        AstAction::Mult,
        year_leaf.into(),
        thousand_leaf.into(),
        AstPtr::ConvDate,
    );
    let month_node = compiler_context.ast.create_node(
        AstAction::Mult,
        month_leaf.into(),
        hundread_leaf.into(),
        AstPtr::ConvDate,
    );
    let day_node = compiler_context.ast.create_node(
        AstAction::Mult,
        day_leaf.into(),
        one_leaf.into(),
        AstPtr::ConvDate,
    );

    let sum_year_month_node = compiler_context.ast.create_node(
        AstAction::Plus,
        year_node.into(),
        month_node.into(),
        AstPtr::ConvDate,
    );
    compiler_context.ast.create_node(
        AstAction::Plus,
        sum_year_month_node.into(),
        day_node.into(),
        AstPtr::ConvDate,
    );

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
    compiler_context: &mut CompilerContext,
) -> VarDeclarations {
    var_declaration.push_to_symbol_table(compiler_context);
    compiler_context.write_to_parser_file("<VarDeclarations> -> <VarDeclaration>");
    VarDeclarations::VarDeclarationsSingle(var_declaration)
}

/// Parses the `<VarDeclarations> -> <VarDeclaration> <VarDeclarations>`
pub fn var_declarations_var_declarations_recursive(
    _ctx: &Ctx,
    var_declaration: VarDeclaration,
    var_declarations: VarDeclarations,
    compiler_context: &mut CompilerContext,
) -> VarDeclarations {
    var_declaration.push_to_symbol_table(compiler_context);
    compiler_context
        .write_to_parser_file("<VarDeclarations> -> <VarDeclaration> <VarDeclarations>");
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
    compiler_context: &mut CompilerContext,
) -> VarDeclaration {
    compiler_context.write_to_parser_file(&format!(
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
    compiler_context: &mut CompilerContext,
) -> VarDeclaration {
    compiler_context.write_to_parser_file(&format!(
        "<VarDeclaration> -> {token_id} {token_comma} <VarDeclaration>"
    ));
    VarDeclaration::VarDeclarationRecursive(VarDeclarationRecursive {
        token_id,
        token_comma,
        var_declaration: Box::new(var_declaration),
    })
}

/// Parses the rule `<Expressions> -> <Statement>`
pub fn expressions_expression_single(
    ctx: &Ctx,
    statement: Statement,
    compiler_context: &mut CompilerContext,
) -> Expressions {
    compiler_context.write_to_parser_file("<Expressions> -> <Statement>");
    let Some(statement_node) = compiler_context.ast.statement_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "Statement stack was empty when parsing `<Expressions> -> <Statement>`".into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    compiler_context
        .ast
        .assign_node_to_ptr(statement_node.into(), AstPtr::Expressions);
    Expressions::ExpressionSingle(statement)
}

/// Parses the rule `<Expressions> -> <Statement> <Expressions>`
pub fn expressions_expression_recursive(
    ctx: &Ctx,
    statement: Statement,
    expressions: Expressions,
    compiler_context: &mut CompilerContext,
) -> Expressions {
    compiler_context.write_to_parser_file("<Expressions> -> <Statement> <Expressions>");
    let Some(statement_node) = compiler_context.ast.statement_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "Statement stack was empty when parsing `<Expressions> -> <Statement> <Expressions>`".into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    compiler_context.ast.create_node(
        AstAction::S,
        AstPtr::Expressions.into(),
        statement_node.into(),
        AstPtr::Expressions,
    );
    Expressions::ExpressionRecursive(ExpressionRecursive {
        statement,
        expressions: Box::new(expressions),
    })
}

/// Parses the rule `<Statement> -> <Assignment>`
pub fn statement_statement_assignment(
    _ctx: &Ctx,
    assignment: Assignment,
    compiler_context: &mut CompilerContext,
) -> Statement {
    compiler_context.write_to_parser_file("<Statement> -> <Assignment>");
    let assignment_node = compiler_context.ast.get_node_from_ptr(AstPtr::Assignment);
    compiler_context
        .ast
        .assign_node_to_ptr(assignment_node.clone().into(), AstPtr::Statement);
    compiler_context.ast.statement_stack.push(assignment_node);
    Statement::StatementAssignment(assignment)
}

/// Parses the rule `<Statement> -> <IfStatement>`
pub fn statement_statement_if_statement(
    _ctx: &Ctx,
    if_statement: IfStatement,
    compiler_context: &mut CompilerContext,
) -> Statement {
    compiler_context.write_to_parser_file("<Statement> -> <IfStatement>");
    let if_node = compiler_context.ast.get_node_from_ptr(AstPtr::If);
    compiler_context
        .ast
        .assign_node_to_ptr(if_node.clone().into(), AstPtr::Statement);
    compiler_context.ast.statement_stack.push(if_node);
    Statement::StatementIfStatement(if_statement)
}

/// Parses the rule `<Statement> -> <WhileLoop>`
pub fn statement_statement_while(
    _ctx: &Ctx,
    while_loop: WhileLoop,
    compiler_context: &mut CompilerContext,
) -> Statement {
    compiler_context.write_to_parser_file("<Statement> -> <WhileLoop>");
    let while_node = compiler_context.ast.get_node_from_ptr(AstPtr::While);
    compiler_context
        .ast
        .assign_node_to_ptr(while_node.clone().into(), AstPtr::Statement);
    compiler_context.ast.statement_stack.push(while_node);
    Statement::StatementWhile(while_loop)
}

/// Parses the rule `<Statement> -> <FunctionWrite>`
pub fn statement_statement_write(
    _ctx: &Ctx,
    function_write: FunctionWrite,
    compiler_context: &mut CompilerContext,
) -> Statement {
    compiler_context.write_to_parser_file("<Statement> -> <FunctionWrite>");
    let write_node = compiler_context.ast.get_node_from_ptr(AstPtr::Write);
    compiler_context
        .ast
        .assign_node_to_ptr(write_node.clone().into(), AstPtr::Statement);
    compiler_context.ast.statement_stack.push(write_node);
    Statement::StatementWrite(function_write)
}

/// Parses the rule `<Statement> -> <FunctionRead>`
pub fn statement_statement_read(
    _ctx: &Ctx,
    function_read: FunctionRead,
    compiler_context: &mut CompilerContext,
) -> Statement {
    compiler_context.write_to_parser_file("<Statement> -> <FunctionRead>");
    let read_node = compiler_context.ast.get_node_from_ptr(AstPtr::Read);
    compiler_context
        .ast
        .assign_node_to_ptr(read_node.clone().into(), AstPtr::Statement);
    compiler_context.ast.statement_stack.push(read_node);
    Statement::StatementRead(function_read)
}

/// Parses the rule `<Assignment> -> TokenId TokenAssign <SimpleExpression>`
pub fn assignment_assignment_expression(
    _ctx: &Ctx,
    token_id: TokenId,
    token_assign: TokenAssign,
    simple_expression: SimpleExpression,
    compiler_context: &mut CompilerContext,
) -> Assignment {
    compiler_context.write_to_parser_file(&format!(
        "<Assignment> -> {token_id} {token_assign} <SimpleExpression>"
    ));
    let leaf = Node::new_leaf(NodeValue::Value(token_id.clone()));
    compiler_context.ast.create_node(
        AstAction::Assign,
        Rc::new(leaf).into(),
        AstPtr::SimpleExpression.into(),
        AstPtr::Assignment,
    );
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
    compiler_context: &mut CompilerContext,
) -> Assignment {
    compiler_context.write_to_parser_file(&format!(
        "<Assignment> -> {token_id} {token_assign} <FunctionConvDate>"
    ));
    let leaf = Rc::new(Node::new_leaf(NodeValue::Value(token_id.clone())));
    compiler_context.ast.create_node(
        AstAction::Assign,
        leaf.into(),
        AstPtr::ConvDate.into(),
        AstPtr::Assignment,
    );
    Assignment::AssignmentConvDate(ConvDate {
        token_id,
        token_assign,
        function_conv_date,
    })
}

/// Parses the rule `<DataType> -> "int"`
pub fn data_type_int_type(
    _ctx: &Ctx,
    token_int: TokenInt,
    compiler_context: &mut CompilerContext,
) -> DataType {
    compiler_context.write_to_parser_file(&format!("<DataType> -> {token_int}"));
    DataType::IntType(token_int)
}

/// Parses the rule `<DataType> -> "float"`
pub fn data_type_float_type(
    _ctx: &Ctx,
    token_float: TokenFloat,
    compiler_context: &mut CompilerContext,
) -> DataType {
    compiler_context.write_to_parser_file(&format!("<DataType> -> {token_float}"));
    DataType::FloatType(token_float)
}

/// Parses the rule `<DataType> -> "string"`
pub fn data_type_string_type(
    _ctx: &Ctx,
    token_string: TokenString,
    compiler_context: &mut CompilerContext,
) -> DataType {
    compiler_context.write_to_parser_file(&format!("<DataType> -> {token_string}"));
    DataType::StringType(token_string)
}

/// Parses the rule `<WhileLoop> -> TokenWhile TokenParOpen <Conjunction> TokenParClose TokenCBOpen <Body> TokenCBClose`
#[expect(clippy::too_many_arguments)]
pub fn while_loop_while(
    ctx: &Ctx,
    token_while: TokenWhile,
    token_par_open: TokenParOpen,
    conjunction: Conjunction,
    token_par_close: TokenParClose,
    token_cbopen: TokenCBOpen,
    body: Body,
    token_cbclose: TokenCBClose,
    compiler_context: &mut CompilerContext,
) -> WhileLoop {
    compiler_context.write_to_parser_file(&format!(
        "<WhileLoop> -> {token_while} {token_par_open} <Conjunction> {token_par_close} {token_cbopen} <Body> {token_cbclose}"
    ));
    let Some(conjunction_node) = compiler_context.ast.conjunction_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "Conjunction stack was empty when parsing `<WhileLoop> -> TokenWhile TokenParOpen <Conjunction> TokenParClose TokenCBOpen <Body> TokenCBClose`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    compiler_context.ast.create_node(
        AstAction::While,
        conjunction_node.into(),
        AstPtr::Body.into(),
        AstPtr::While,
    );
    WhileLoop {
        token_while,
        token_par_open,
        conjunction,
        token_par_close,
        token_cbopen,
        body: Box::new(body),
        token_cbclose,
    }
}

/// Parses the rule `<IfStatement> -> TokenIf TokenParOpen <Conjunction> TokenParClose TokenCBOpen <Body> TokenCBClose`
#[expect(clippy::too_many_arguments)]
pub fn if_statement_if_statement(
    ctx: &Ctx,
    token_if: TokenIf,
    token_par_open: TokenParOpen,
    conjunction: Conjunction,
    token_par_close: TokenParClose,
    token_cbopen: TokenCBOpen,
    body: Body,
    token_cbclose: TokenCBClose,
    compiler_context: &mut CompilerContext,
) -> IfStatement {
    compiler_context.write_to_parser_file(&format!(
        "<IfStatement> -> {token_if} {token_par_open} <Conjunction> {token_par_close} {token_cbopen} <Body> {token_cbclose}"
    ));
    let Some(conjunction_node) = compiler_context.ast.conjunction_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "Conjunction stack was empty when parsing `<IfStatement> -> TokenIf TokenParOpen <Conjunction> TokenParClose TokenCBOpen <Body> TokenCBClose`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    compiler_context.ast.create_node(
        AstAction::If,
        conjunction_node.into(),
        AstPtr::Body.into(),
        AstPtr::If,
    );
    IfStatement::IfStatementIfStatement(IfStatementIfStatement {
        token_if,
        token_par_open,
        conjunction,
        token_par_close,
        token_cbopen,
        body: Box::new(body),
        token_cbclose,
    })
}

/// Parses the rule `<IfStatement> -> TokenIf TokenParOpen <Conjunction> TokenParClose TokenCBOpen <Body> TokenCBClose <DummyElse> <ElseStatement>`
#[expect(clippy::too_many_arguments)]
pub fn if_statement_if_statement_else_statement(
    ctx: &Ctx,
    token_if: TokenIf,
    token_par_open: TokenParOpen,
    conjunction: Conjunction,
    token_par_close: TokenParClose,
    token_cbopen: TokenCBOpen,
    body: Body,
    token_cbclose: TokenCBClose,
    else_statement: ElseStatement,
    compiler_context: &mut CompilerContext,
) -> IfStatement {
    compiler_context.write_to_parser_file(&format!(
        "<IfStatement> -> {token_if} {token_par_open} <Conjunction> {token_par_close} {token_cbopen} <Body> {token_cbclose} <DummyElse> <ElseStatement>"
    ));
    let Some(if_true_body) = compiler_context.ast.if_body_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "IfBody stack was empty when parsing `<IfStatement> -> TokenIf TokenParOpen <Conjunction> TokenParClose TokenCBOpen <Body> TokenCBClose <DummyElse> <ElseStatement>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    let else_node = compiler_context.ast.create_node(
        AstAction::Else,
        if_true_body.into(),
        AstPtr::Body.into(),
        AstPtr::If,
    );
    let Some(conjunction_node) = compiler_context.ast.conjunction_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "Conjunction stack was empty when parsing `<IfStatement> -> TokenIf TokenParOpen <Conjunction> TokenParClose TokenCBOpen <Body> TokenCBClose <DummyElse> <ElseStatement>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    compiler_context.ast.create_node(
        AstAction::If,
        conjunction_node.into(),
        else_node.into(),
        AstPtr::If,
    );
    IfStatement::IfStatementElseStatement(IfStatementElseStatement {
        token_if,
        token_par_open,
        conjunction,
        token_par_close,
        token_cbopen,
        body: Box::new(body),
        token_cbclose,
        else_statement: Box::new(else_statement),
    })
}

/// Parses the rule `<DummyElse> -> EMPTY`
pub fn dummy_else_empty(_ctx: &Ctx, compiler_context: &mut CompilerContext) -> DummyElse {
    compiler_context.write_to_parser_file("<DummyElse> -> EMPTY");
    let body_node = compiler_context.ast.get_node_from_ptr(AstPtr::Body);
    compiler_context.ast.if_body_stack.push(body_node);
    None
}

/// Parses the rule `<ElseStatement> -> TokenElse TokenCBOpen <Body> TokenCBClose`
pub fn else_statement_else_statement(
    _ctx: &Ctx,
    token_else: TokenElse,
    token_cbopen: TokenCBOpen,
    body: Body,
    token_cbclose: TokenCBClose,
    compiler_context: &mut CompilerContext,
) -> ElseStatement {
    compiler_context.write_to_parser_file(&format!(
        "<ElseStatement> -> {token_else} {token_cbopen} <Body> {token_cbclose}"
    ));
    ElseStatement {
        token_else,
        token_cbopen,
        body: Box::new(body),
        token_cbclose,
    }
}

/// Parses the rule `<BooleanExpression> -> <SimpleExpression> <ComparisonOp> <SimpleExpression>`
pub fn boolean_expression_boolean_expression_simple_expression(
    ctx: &Ctx,
    simple_expression: SimpleExpression,
    comparison_op: ComparisonOp,
    simple_expression_2: SimpleExpression,
    compiler_context: &mut CompilerContext,
) -> BooleanExpression {
    compiler_context.write_to_parser_file(
        "<BooleanExpression> -> <SimpleExpression> <ComparisonOp> <SimpleExpression>",
    );
    let Some(left_child) = compiler_context.ast.comparision_expressions_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "ComparisonExpressions stack was empty when parsing `<BooleanExpression> -> <SimpleExpression> <ComparisonOp> <SimpleExpression>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    let Some(operator) = compiler_context.ast.comparision_op_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "ComparisonOperator stack was empty when parsing `<BooleanExpression> -> <SimpleExpression> <ComparisonOp> <SimpleExpression>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    let node = compiler_context.ast.create_node(
        operator.into(),
        left_child.into(),
        AstPtr::SimpleExpression.into(),
        AstPtr::BooleanExpression,
    );
    compiler_context.ast.boolean_expression_stack.push(node);
    BooleanExpression::BooleanExpressionSimpleExpression(BooleanExpressionSimpleExpression {
        simple_expression,
        comparison_op,
        simple_expression_2,
    })
}

/// Parses the rule `<BooleanExpression> -> "true"`
pub fn boolean_expression_boolean_expression_true(
    _ctx: &Ctx,
    token_true: TokenTrue,
    compiler_context: &mut CompilerContext,
) -> BooleanExpression {
    compiler_context.write_to_parser_file(&format!("<BooleanExpression> -> {token_true}"));
    let node = compiler_context
        .ast
        .create_leaf(token_true.clone(), AstPtr::BooleanExpression);
    compiler_context.ast.boolean_expression_stack.push(node);
    BooleanExpression::BooleanExpressionTrue(token_true)
}

/// Parses the rule `<BooleanExpression> -> "false"`
pub fn boolean_expression_boolean_expression_false(
    _ctx: &Ctx,
    token_false: TokenFalse,
    compiler_context: &mut CompilerContext,
) -> BooleanExpression {
    compiler_context.write_to_parser_file(&format!("<BooleanExpression> -> {token_false}"));
    let node = compiler_context
        .ast
        .create_leaf(token_false.clone(), AstPtr::BooleanExpression);
    compiler_context.ast.boolean_expression_stack.push(node);
    BooleanExpression::BooleanExpressionFalse(token_false)
}

/// Parses the rule `<BooleanExpression> -> TokenId
pub fn boolean_expression_boolean_expression_token_id(
    _ctx: &Ctx,
    token_id: TokenId,
    compiler_context: &mut CompilerContext,
) -> BooleanExpression {
    compiler_context.write_to_parser_file(&format!("<BooleanExpression> -> {token_id}"));
    let node = compiler_context
        .ast
        .create_leaf(token_id.clone(), AstPtr::BooleanExpression);
    compiler_context.ast.boolean_expression_stack.push(node);
    BooleanExpression::BooleanExpressionTokenId(token_id)
}

/// Parses the rule `<BooleanExpression> -> <NotStatement>`
pub fn boolean_expression_boolean_expression_not_statement(
    _ctx: &Ctx,
    not_statement: NotStatement,
    compiler_context: &mut CompilerContext,
) -> BooleanExpression {
    compiler_context.write_to_parser_file("<BooleanExpression> -> <NotStatement>");
    let node = compiler_context.ast.get_node_from_ptr(AstPtr::Not);
    compiler_context.ast.boolean_expression_stack.push(node);
    BooleanExpression::BooleanExpressionNotStatement(not_statement)
}

/// Parses thre rule `<BooleanExpression> -> <FunctionIsZero>`
pub fn boolean_expression_boolean_expression_is_zero(
    _ctx: &Ctx,
    function_is_zero: FunctionIsZero,
    compiler_context: &mut CompilerContext,
) -> BooleanExpression {
    compiler_context.write_to_parser_file("<BooleanExpression> -> <FunctionIsZero>");
    let node = compiler_context.ast.get_node_from_ptr(AstPtr::IsZero);
    compiler_context.ast.boolean_expression_stack.push(node);
    BooleanExpression::BooleanExpressionIsZero(function_is_zero)
}

/// Parses the rule `<SimpleExpression> -> <ArithmeticExpression>`
pub fn simple_expression_simple_expression_arithmetic(
    _ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
    compiler_context: &mut CompilerContext,
) -> SimpleExpression {
    compiler_context.write_to_parser_file("<SimpleExpression> -> <ArithmeticExpression>");
    compiler_context.ast.assign_node_to_ptr(
        AstPtr::ArithmeticExpression.into(),
        AstPtr::SimpleExpression,
    );
    SimpleExpression::SimpleExpressionArithmeticExpression(arithmetic_expression)
}

/// Parses the rule `<SimpleExpression> -> TokenStringLiteral`
pub fn simple_expression_simple_expression_string(
    _ctx: &Ctx,
    token_string_literal: TokenStringLiteral,
    compiler_context: &mut CompilerContext,
) -> SimpleExpression {
    compiler_context.push_to_symbol_table(token_string_literal.clone().into());
    compiler_context.write_to_parser_file(&format!("<SimpleExpression> -> {token_string_literal}"));
    compiler_context
        .ast
        .create_leaf(token_string_literal.clone(), AstPtr::SimpleExpression);
    SimpleExpression::SimpleExpressionString(token_string_literal)
}

/// Parses the rule `<Conjunction> -> <BooleanExpression> "and" <Conjunction>`
pub fn conjunction_conjunction_and(
    ctx: &Ctx,
    boolean_expression: BooleanExpression,
    token_and: TokenAnd,
    conjunction: Conjunction,
    compiler_context: &mut CompilerContext,
) -> Conjunction {
    compiler_context.write_to_parser_file(&format!(
        "<Conjunction> -> <BooleanExpression> {token_and} <Conjunction>"
    ));
    let Some(boolean_expression_node) = compiler_context.ast.boolean_expression_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "BooleanExpression stack was empty when parsing `<Conjunction> -> <BooleanExpression> \"and\" <Conjunction>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };

    let Some(conjunction_node) = compiler_context.ast.conjunction_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "Conjunction stack was empty when parsing `<Conjunction> -> <BooleanExpression> \"and\" <Conjunction>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    let conjunction_node = compiler_context.ast.create_node(
        AstAction::And,
        boolean_expression_node.into(),
        conjunction_node.into(),
        AstPtr::Conjunction,
    );
    compiler_context
        .ast
        .conjunction_stack
        .push(conjunction_node);

    Conjunction::ConjunctionAnd(ConjunctionAnd {
        boolean_expression,
        token_and,
        conjunction: Box::new(conjunction),
    })
}

/// Parses the rule `<Conjunction> -> <BooleanExpression> "or" <Conjunction>`
pub fn conjunction_conjunction_or(
    ctx: &Ctx,
    boolean_expression: BooleanExpression,
    token_or: TokenOr,
    conjunction: Conjunction,
    compiler_context: &mut CompilerContext,
) -> Conjunction {
    compiler_context.write_to_parser_file(&format!(
        "<Conjunction> -> <BooleanExpression> {token_or} <Conjunction>"
    ));
    let Some(boolean_expression_node) = compiler_context.ast.boolean_expression_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "BooleanExpression stack was empty when parsing `<Conjunction> -> <BooleanExpression> \"or\" <Conjunction>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    let Some(conjunction_node) = compiler_context.ast.conjunction_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "Conjunction stack was empty when parsing `<Conjunction> -> <BooleanExpression> \"or\" <Conjunction>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    let conjunction_node = compiler_context.ast.create_node(
        AstAction::Or,
        boolean_expression_node.into(),
        conjunction_node.into(),
        AstPtr::Conjunction,
    );
    compiler_context
        .ast
        .conjunction_stack
        .push(conjunction_node);
    Conjunction::ConjunctionOr(ConjunctionOr {
        boolean_expression,
        token_or,
        conjunction: Box::new(conjunction),
    })
}

/// Parses the rule `<Conjunction> -> <BooleanExpression>`
pub fn conjunction_conjunction_boolean_expression(
    ctx: &Ctx,
    boolean_expression: BooleanExpression,
    compiler_context: &mut CompilerContext,
) -> Conjunction {
    compiler_context.write_to_parser_file("<Conjunction> -> <BooleanExpression>");
    let Some(boolean_expression_node) = compiler_context.ast.boolean_expression_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "BooleanExpression stack was empty when parsing `<Conjunction> -> <BooleanExpression>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    compiler_context
        .ast
        .conjunction_stack
        .push(boolean_expression_node);
    Conjunction::ConjunctionBooleanExpression(boolean_expression)
}

/// Parses the rule `<ComparisonOp> -> "=="`
pub fn comparison_op_comparison_op_equal(
    _ctx: &Ctx,
    token_equal: TokenEqual,
    compiler_context: &mut CompilerContext,
) -> ComparisonOp {
    compiler_context.write_to_parser_file(&format!("<ComparisonOp> -> {token_equal}"));
    let result = ComparisonOp::ComparisonOpEqual(token_equal);
    compiler_context
        .ast
        .comparision_op_stack
        .push(result.clone());
    compiler_context.ast.comparision_expressions_stack.push(
        compiler_context
            .ast
            .get_node_from_ptr(AstPtr::SimpleExpression),
    );
    result
}

/// Parses the rule  `<ComparisonOp> -> "!="`
pub fn comparison_op_comparison_op_not_equal(
    _ctx: &Ctx,
    token_not_equal: TokenNotEqual,
    compiler_context: &mut CompilerContext,
) -> ComparisonOp {
    compiler_context.write_to_parser_file(&format!("<ComparisonOp> -> {token_not_equal}"));
    let result = ComparisonOp::ComparisonOpNotEqual(token_not_equal);
    compiler_context
        .ast
        .comparision_op_stack
        .push(result.clone());
    compiler_context.ast.comparision_expressions_stack.push(
        compiler_context
            .ast
            .get_node_from_ptr(AstPtr::SimpleExpression),
    );
    result
}

/// Parses the rule `<ComparisonOp> -> "<"`
pub fn comparison_op_comparison_op_less(
    _ctx: &Ctx,
    token_less: TokenLess,
    compiler_context: &mut CompilerContext,
) -> ComparisonOp {
    compiler_context.write_to_parser_file(&format!("<ComparisonOp> -> {token_less}"));
    let result = ComparisonOp::ComparisonOpLess(token_less);
    compiler_context
        .ast
        .comparision_op_stack
        .push(result.clone());
    compiler_context.ast.comparision_expressions_stack.push(
        compiler_context
            .ast
            .get_node_from_ptr(AstPtr::SimpleExpression),
    );
    result
}

/// Parses the rule `<ComparisonOp> -> ">="`
pub fn comparison_op_comparison_op_less_equal(
    _ctx: &Ctx,
    token_less_equal: TokenLessEqual,
    compiler_context: &mut CompilerContext,
) -> ComparisonOp {
    compiler_context.write_to_parser_file(&format!("<ComparisonOp> -> {token_less_equal}"));
    let result = ComparisonOp::ComparisonOpLessEqual(token_less_equal);
    compiler_context
        .ast
        .comparision_op_stack
        .push(result.clone());
    compiler_context.ast.comparision_expressions_stack.push(
        compiler_context
            .ast
            .get_node_from_ptr(AstPtr::SimpleExpression),
    );
    result
}

/// Parses the rule `<ComparisonOp> -> ">"`
pub fn comparison_op_comparison_op_greater(
    _ctx: &Ctx,
    token_greater: TokenGreater,
    compiler_context: &mut CompilerContext,
) -> ComparisonOp {
    compiler_context.write_to_parser_file(&format!("<ComparisonOp> -> {token_greater}"));
    let result = ComparisonOp::ComparisonOpGreater(token_greater);
    compiler_context
        .ast
        .comparision_op_stack
        .push(result.clone());
    compiler_context.ast.comparision_expressions_stack.push(
        compiler_context
            .ast
            .get_node_from_ptr(AstPtr::SimpleExpression),
    );
    result
}

/// Parses the rule `<ComparisonOp> -> ">="`
pub fn comparison_op_comparison_op_greater_equal(
    _ctx: &Ctx,
    token_greater_equal: TokenGreaterEqual,
    compiler_context: &mut CompilerContext,
) -> ComparisonOp {
    compiler_context.write_to_parser_file(&format!("<ComparisonOp> -> {token_greater_equal}"));
    let result = ComparisonOp::ComparisonOpGreaterEqual(token_greater_equal);
    compiler_context
        .ast
        .comparision_op_stack
        .push(result.clone());
    compiler_context.ast.comparision_expressions_stack.push(
        compiler_context
            .ast
            .get_node_from_ptr(AstPtr::SimpleExpression),
    );
    result
}

/// Parses the rule `<Number> -> TokenIntLiteral`
pub fn number_number_int(
    _ctx: &Ctx,
    token_int_literal: TokenIntLiteral,
    compiler_context: &mut CompilerContext,
) -> Number {
    compiler_context.push_to_symbol_table(token_int_literal.into());
    compiler_context.write_to_parser_file(&format!("<Number> -> {token_int_literal}"));
    compiler_context
        .ast
        .create_leaf(token_int_literal.to_string(), AstPtr::Number);
    Number::NumberInt(token_int_literal)
}

/// Parses the rule `<Number> -> TokenFloatLiteral`
pub fn number_number_float(
    _ctx: &Ctx,
    token_float_literal: TokenFloatLiteral,
    compiler_context: &mut CompilerContext,
) -> Number {
    compiler_context.push_to_symbol_table(token_float_literal.clone().into());
    compiler_context.write_to_parser_file(&format!("<Number> -> {}", token_float_literal.original));
    compiler_context
        .ast
        .create_leaf(token_float_literal.original.clone(), AstPtr::Number);
    Number::NumberFloat(token_float_literal)
}

/// Parses the rule `<Number> -> TokenSub TokenIntLiteral`
pub fn number_number_negative_int(
    _ctx: &Ctx,
    token_sub: TokenSub,
    token_int_literal: TokenIntLiteral,
    compiler_context: &mut CompilerContext,
) -> Number {
    let value: i64 = format!("{token_sub}{token_int_literal}").parse().unwrap();
    compiler_context.push_to_symbol_table(value.into());
    compiler_context.write_to_parser_file(&format!("<Number> -> {token_sub} {token_int_literal}"));
    let leaf = Rc::new(Node::new_leaf(NodeValue::Value(
        token_int_literal.to_string(),
    )));
    let noop = Rc::new(Node::new_leaf(NodeValue::Action(AstAction::Noop)));
    compiler_context.ast.create_node(
        AstAction::Negative,
        leaf.into(),
        noop.into(),
        AstPtr::Number,
    );
    Number::NumberInt(value)
}

/// Parses the rule `<Number> -> TokenSub TokenFloatLiteral`
pub fn number_number_negative_float(
    _ctx: &Ctx,
    token_sub: TokenSub,
    mut token_float_literal: TokenFloatLiteral,
    compiler_context: &mut CompilerContext,
) -> Number {
    token_float_literal.original = format!("{token_sub}{}", token_float_literal.original);
    token_float_literal.parsed *= -1_f32;
    compiler_context.push_to_symbol_table(token_float_literal.clone().into());
    compiler_context.write_to_parser_file(&format!(
        "<Number> -> {token_sub} {}",
        token_float_literal.original
    ));
    let leaf = Rc::new(Node::new_leaf(NodeValue::Value(
        token_float_literal.original.clone(),
    )));
    let noop = Rc::new(Node::new_leaf(NodeValue::Action(AstAction::Noop)));
    compiler_context.ast.create_node(
        AstAction::Negative,
        leaf.into(),
        noop.into(),
        AstPtr::Number,
    );

    Number::NumberFloat(token_float_literal)
}

/// Parses the rule `<NotStatement> -> TokenNot <BooleanExpression>`
pub fn not_statement_not(
    ctx: &Ctx,
    token_not: TokenNot,
    boolean_expression: BooleanExpression,
    compiler_context: &mut CompilerContext,
) -> NotStatement {
    compiler_context.write_to_parser_file(&format!(
        "<NotStatement> -> {token_not} <BooleanExpression>"
    ));
    let Some(boolean_expression_node) = compiler_context.ast.boolean_expression_stack.pop() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "BooleanExpression stack was empty when parsing `<NotStatement> -> TokenNot <BooleanExpression>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };

    let dummy = Node::new_leaf(NodeValue::Action(AstAction::Noop));
    compiler_context.ast.create_node(
        AstAction::Not,
        boolean_expression_node.into(),
        Rc::new(dummy).into(),
        AstPtr::Not,
    );
    NotStatement {
        token_not,
        boolean_expression: Box::new(boolean_expression),
    }
}

/// Parses the rule `<ArithmeticExpression> -> <ArithmeticExpression> <DummyAE> TokenSum <Term>`
pub fn arithmetic_expression_arithmetic_expression_sum_term(
    ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
    token_sum: TokenSum,
    term: Term,
    compiler_context: &mut CompilerContext,
) -> ArithmeticExpression {
    compiler_context.write_to_parser_file(&format!(
        "<ArithmeticExpression> -> <ArithmeticExpression> <DummyAE> {token_sum} <Term>"
    ));
    let Some(node) = compiler_context.ast.pop_e_stack() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "ArithmeticExpression stack was empty when parsing `<ArithmeticExpression> -> <ArithmeticExpression> <DummyAE> TokenSum <Term>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    compiler_context.ast.create_node(
        AstAction::Plus,
        node.into(),
        AstPtr::Term.into(),
        AstPtr::ArithmeticExpression,
    );
    ArithmeticExpression::ArithmeticExpressionSumTerm(ArithmeticExpressionSumTerm {
        arithmetic_expression: Box::new(arithmetic_expression),
        token_sum,
        term,
    })
}

/// Parses the rule `<ArithmeticExpression> -> <ArithmeticExpression> <DummyAE> TokenSub <Term>`
pub fn arithmetic_expression_arithmetic_expression_sub_term(
    ctx: &Ctx,
    arithmetic_expression: ArithmeticExpression,
    token_sub: TokenSub,
    term: Term,
    compiler_context: &mut CompilerContext,
) -> ArithmeticExpression {
    compiler_context.write_to_parser_file(&format!(
        "<ArithmeticExpression> -> <ArithmeticExpression> <DummyAE> {token_sub} <Term>"
    ));
    let Some(node) = compiler_context.ast.pop_e_stack() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "ArithmeticExpression stack was empty when parsing `<ArithmeticExpression> -> <ArithmeticExpression> <DummyAE> TokenSub <Term>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    compiler_context.ast.create_node(
        AstAction::Sub,
        node.into(),
        AstPtr::Term.into(),
        AstPtr::ArithmeticExpression,
    );
    ArithmeticExpression::ArithmeticExpressionSubTerm(ArithmeticExpressionSubTerm {
        arithmetic_expression: Box::new(arithmetic_expression),
        token_sub,
        term,
    })
}

// Parses the rule `<DummyAE> -> EMPTY`
pub fn dummy_ae_empty(_ctx: &Ctx, compiler_context: &mut CompilerContext) -> DummyAE {
    compiler_context
        .ast
        .push_e_stack(AstPtr::ArithmeticExpression.into());
    None
}

/// Parses the rule `<ArithmeticExpression> -> <Term>`
pub fn arithmetic_expression_arithmetic_expression_term(
    _ctx: &Ctx,
    term: Term,
    compiler_context: &mut CompilerContext,
) -> ArithmeticExpression {
    compiler_context.write_to_parser_file("<ArithmeticExpression> -> <Term>");
    compiler_context
        .ast
        .assign_node_to_ptr(AstPtr::Term.into(), AstPtr::ArithmeticExpression);
    ArithmeticExpression::ArithmeticExpressionTerm(term)
}

/// Parses the rule `<Term> -> <Term> <DummyT> TokenMul <Factor>`
pub fn term_term_mul_factor(
    ctx: &Ctx,
    term: Term,
    token_mul: TokenMul,
    factor: Factor,
    compiler_context: &mut CompilerContext,
) -> Term {
    compiler_context
        .write_to_parser_file(&format!("<Term> -> <Term> <DummyT> {token_mul} <Factor>"));
    let Some(node) = compiler_context.ast.pop_t_stack() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "Term stack was empty when parsing `<Term> -> <Term> <DummyT> TokenMul <Factor>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    compiler_context.ast.create_node(
        AstAction::Mult,
        node.into(),
        AstPtr::Factor.into(),
        AstPtr::Term,
    );
    Term::TermMulFactor(TermMulFactor {
        term: Box::new(term),
        token_mul,
        factor,
    })
}

/// Parses the rule `<Term> -> <Term> <DummyT> TokenDiv <Factor>`
pub fn term_term_div_factor(
    ctx: &Ctx,
    term: Term,
    token_div: TokenDiv,
    factor: Factor,
    compiler_context: &mut CompilerContext,
) -> Term {
    compiler_context
        .write_to_parser_file(&format!("<Term> -> <Term> <DummyT> {token_div} <Factor>"));
    let Some(node) = compiler_context.ast.pop_t_stack() else {
        log_error_and_exit(
            ctx.range(),
            CompilerError::Internal(
                "Term stack was empty when parsing `<Term> -> <Term> <DummyT> TokenDiv <Factor>`"
                    .into(),
            ),
            0,
            true,
            compiler_context,
        )
    };
    compiler_context.ast.create_node(
        AstAction::Div,
        node.into(),
        AstPtr::Factor.into(),
        AstPtr::Term,
    );
    Term::TermDivFactor(TermDivFactor {
        term: Box::new(term),
        token_div,
        factor,
    })
}

// Parses the rule `<DummyT> -> EMPTY`
pub fn dummy_t_empty(_ctx: &Ctx, compiler_context: &mut CompilerContext) -> DummyT {
    compiler_context.ast.push_t_stack(AstPtr::Term.into());
    None
}

/// Parses the rule `<Term> -> <Factor>`
pub fn term_term_factor(
    _ctx: &Ctx,
    factor: Factor,
    compiler_context: &mut CompilerContext,
) -> Term {
    compiler_context.write_to_parser_file("<Term> -> <Factor>");
    compiler_context
        .ast
        .assign_node_to_ptr(AstPtr::Factor.into(), AstPtr::Term);
    Term::TermFactor(factor)
}

/// Parses the rule `<Factor> -> TokenId`
pub fn factor_factor_id(
    _ctx: &Ctx,
    token_id: TokenId,
    compiler_context: &mut CompilerContext,
) -> Factor {
    compiler_context.write_to_parser_file(&format!("<Factor> -> {token_id}"));
    compiler_context
        .ast
        .create_leaf(token_id.clone(), AstPtr::Factor);
    Factor::FactorId(token_id)
}

/// Parses the rule `<Factor> -> <Number>`
pub fn factor_factor_number(
    _ctx: &Ctx,
    number: Number,
    compiler_context: &mut CompilerContext,
) -> Factor {
    compiler_context.write_to_parser_file("<Factor> -> <Number>");
    compiler_context
        .ast
        .assign_node_to_ptr(AstPtr::Number.into(), AstPtr::Factor);
    Factor::FactorNumber(number)
}

/// Parses the rule `<Factor> -> TokenParOpen <ArithmeticExpression> TokenParClose`
pub fn factor_factor_paren(
    _ctx: &Ctx,
    token_par_open: TokenParOpen,
    arithmetic_expression: ArithmeticExpression,
    token_par_close: TokenParClose,
    compiler_context: &mut CompilerContext,
) -> Factor {
    compiler_context.write_to_parser_file(&format!(
        "<Factor> -> {token_par_open} <ArithmeticExpression> {token_par_close}"
    ));
    compiler_context
        .ast
        .assign_node_to_ptr(AstPtr::ArithmeticExpression.into(), AstPtr::Factor);
    Factor::FactorParen(FactorParen {
        token_par_open,
        arithmetic_expression: Box::new(arithmetic_expression),
        token_par_close,
    })
}
