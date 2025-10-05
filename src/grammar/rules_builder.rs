use crate::compiler::context::Compiler;
use crate::grammar::rules::{Context, Input, ProdKind, State};
use crate::grammar::{TokenKind, rules_actions};
use rustemo::{Builder, LRBuilder, Token};

#[expect(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}

#[derive(Debug)]
pub enum Terminal {
    TokenInt(rules_actions::TokenInt),
    TokenFloat(rules_actions::TokenFloat),
    TokenString(rules_actions::TokenString),
    TokenIntLiteral(rules_actions::TokenIntLiteral),
    TokenFloatLiteral(rules_actions::TokenFloatLiteral),
    TokenStringLiteral(rules_actions::TokenStringLiteral),
    TokenId(rules_actions::TokenId),
    TokenAssign(rules_actions::TokenAssign),
    TokenSum(rules_actions::TokenSum),
    TokenMul(rules_actions::TokenMul),
    TokenSub(rules_actions::TokenSub),
    TokenDiv(rules_actions::TokenDiv),
    TokenParOpen(rules_actions::TokenParOpen),
    TokenParClose(rules_actions::TokenParClose),
    TokenCBOpen(rules_actions::TokenCBOpen),
    TokenCBClose(rules_actions::TokenCBClose),
    TokenColon(rules_actions::TokenColon),
    TokenInit(rules_actions::TokenInit),
    TokenWhile(rules_actions::TokenWhile),
    TokenEqual(rules_actions::TokenEqual),
    TokenNotEqual(rules_actions::TokenNotEqual),
    TokenLess(rules_actions::TokenLess),
    TokenLessEqual(rules_actions::TokenLessEqual),
    TokenGreater(rules_actions::TokenGreater),
    TokenGreaterEqual(rules_actions::TokenGreaterEqual),
    TokenTrue(rules_actions::TokenTrue),
    TokenFalse(rules_actions::TokenFalse),
    TokenIf(rules_actions::TokenIf),
    TokenElse(rules_actions::TokenElse),
    TokenComma(rules_actions::TokenComma),
    TokenAnd(rules_actions::TokenAnd),
    TokenOr(rules_actions::TokenOr),
    TokenNot(rules_actions::TokenNot),
    TokenRead(rules_actions::TokenRead),
    TokenWrite(rules_actions::TokenWrite),
    TokenIsZero(rules_actions::TokenIsZero),
    TokenConvDate(rules_actions::TokenConvDate),
    TokenDate(rules_actions::TokenDate),
}

#[derive(Debug)]
pub enum NonTerminal {
    Program(rules_actions::Program),
    Body(rules_actions::Body),
    InitBody(rules_actions::InitBody),
    FunctionRead(rules_actions::FunctionRead),
    FunctionWrite(rules_actions::FunctionWrite),
    FunctionIsZero(rules_actions::FunctionIsZero),
    FunctionConvDate(rules_actions::FunctionConvDate),
    VarDeclarations(rules_actions::VarDeclarations),
    VarDeclaration(rules_actions::VarDeclaration),
    Expressions(rules_actions::Expressions),
    Statement(rules_actions::Statement),
    Assignment(rules_actions::Assignment),
    DataType(rules_actions::DataType),
    WhileLoop(rules_actions::WhileLoop),
    IfStatement(rules_actions::IfStatement),
    ElseStatement(rules_actions::ElseStatement),
    BooleanExpression(rules_actions::BooleanExpression),
    BooleanExpressionChain(rules_actions::BooleanExpressionChain),
    SimpleExpression(rules_actions::SimpleExpression),
    Conjunction(rules_actions::Conjunction),
    ComparisonOp(rules_actions::ComparisonOp),
    Number(rules_actions::Number),
    NotStatement(rules_actions::NotStatement),
    ArithmeticExpression(rules_actions::ArithmeticExpression),
    Term(rules_actions::Term),
    Factor(rules_actions::Factor),
}

impl Builder for Compiler {
    type Output = String;
    fn get_result(&mut self) -> Self::Output {
        let mut compiler_context = self.inner.borrow_mut();

        if let Err(e) = compiler_context.dump_symbol_table_to_file() {
            eprintln!("Failed to write symbol table to file {e}")
        }

        match compiler_context.res_stack.pop().unwrap() {
            Symbol::NonTerminal(NonTerminal::Program(_r)) => {
                match compiler_context.read_parser_file_to_string() {
                    Err(e) => {
                        eprintln!("Failed to read final list of rules {e}");
                        std::process::exit(1)
                    }
                    Ok(rules) => rules,
                }
            }
            _ => panic!("Invalid result on the parse stack!"),
        }
    }
}

/// This is a copy of the automatically generated implementation of the LRBuilder but implemented
/// for Compiler which holds the compiler context to pass to the parser actions to build the ast while parsing
impl<'i> LRBuilder<'i, Input, Context<'i, Input>, State, ProdKind, TokenKind> for Compiler {
    fn shift_action(&mut self, context: &Context<'i, Input>, token: Token<'i, Input, TokenKind>) {
        let mut compiler_context = self.inner.borrow_mut();
        let val =
            match token.kind {
                TokenKind::STOP => panic!("Cannot shift STOP token!"),
                TokenKind::TokenInt => Terminal::TokenInt(rules_actions::token_int(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenFloat => Terminal::TokenFloat(rules_actions::token_float(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenString => Terminal::TokenString(rules_actions::token_string(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenIntLiteral => Terminal::TokenIntLiteral(
                    rules_actions::token_int_literal(context, token, &mut compiler_context),
                ),
                TokenKind::TokenFloatLiteral => Terminal::TokenFloatLiteral(
                    rules_actions::token_float_literal(context, token, &mut compiler_context),
                ),
                TokenKind::TokenStringLiteral => Terminal::TokenStringLiteral(
                    rules_actions::token_string_literal(context, token, &mut compiler_context),
                ),
                TokenKind::TokenId => Terminal::TokenId(rules_actions::token_id(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenAssign => Terminal::TokenAssign(rules_actions::token_assign(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenSum => Terminal::TokenSum(rules_actions::token_sum(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenMul => Terminal::TokenMul(rules_actions::token_mul(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenSub => Terminal::TokenSub(rules_actions::token_sub(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenDiv => Terminal::TokenDiv(rules_actions::token_div(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenParOpen => Terminal::TokenParOpen(rules_actions::token_par_open(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenParClose => Terminal::TokenParClose(
                    rules_actions::token_par_close(context, token, &mut compiler_context),
                ),
                TokenKind::TokenCBOpen => Terminal::TokenCBOpen(rules_actions::token_cbopen(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenCBClose => Terminal::TokenCBClose(rules_actions::token_cbclose(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenColon => Terminal::TokenColon(rules_actions::token_colon(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenInit => Terminal::TokenInit(rules_actions::token_init(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenWhile => Terminal::TokenWhile(rules_actions::token_while(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenEqual => Terminal::TokenEqual(rules_actions::token_equal(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenNotEqual => Terminal::TokenNotEqual(
                    rules_actions::token_not_equal(context, token, &mut compiler_context),
                ),
                TokenKind::TokenLess => Terminal::TokenLess(rules_actions::token_less(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenLessEqual => Terminal::TokenLessEqual(
                    rules_actions::token_less_equal(context, token, &mut compiler_context),
                ),
                TokenKind::TokenGreater => Terminal::TokenGreater(rules_actions::token_greater(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenGreaterEqual => Terminal::TokenGreaterEqual(
                    rules_actions::token_greater_equal(context, token, &mut compiler_context),
                ),
                TokenKind::TokenTrue => Terminal::TokenTrue(rules_actions::token_true(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenFalse => Terminal::TokenFalse(rules_actions::token_false(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenIf => Terminal::TokenIf(rules_actions::token_if(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenElse => Terminal::TokenElse(rules_actions::token_else(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenComma => Terminal::TokenComma(rules_actions::token_comma(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenAnd => Terminal::TokenAnd(rules_actions::token_and(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenOr => Terminal::TokenOr(rules_actions::token_or(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenNot => Terminal::TokenNot(rules_actions::token_not(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenRead => Terminal::TokenRead(rules_actions::token_read(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenWrite => Terminal::TokenWrite(rules_actions::token_write(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenIsZero => Terminal::TokenIsZero(rules_actions::token_is_zero(
                    context,
                    token,
                    &mut compiler_context,
                )),
                TokenKind::TokenConvDate => Terminal::TokenConvDate(
                    rules_actions::token_conv_date(context, token, &mut compiler_context),
                ),
                TokenKind::TokenDate => Terminal::TokenDate(rules_actions::token_date(
                    context,
                    token,
                    &mut compiler_context,
                )),
            };
        compiler_context.res_stack.push(Symbol::Terminal(val));
    }

    fn reduce_action(&mut self, context: &Context<'i, Input>, prod: ProdKind, _prod_len: usize) {
        let mut compiler_context = self.inner.borrow_mut();
        let stack_len = compiler_context.res_stack.len();
        let prod = match prod {
            ProdKind::ProgramProgramWithMain => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 6usize)
                    .into_iter();
                match (
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                ) {
                    (
                        Symbol::Terminal(Terminal::TokenId(p0)),
                        Symbol::Terminal(Terminal::TokenParOpen(p1)),
                        Symbol::Terminal(Terminal::TokenParClose(p2)),
                        Symbol::Terminal(Terminal::TokenCBOpen(p3)),
                        Symbol::NonTerminal(NonTerminal::Body(p4)),
                        Symbol::Terminal(Terminal::TokenCBClose(p5)),
                    ) => NonTerminal::Program(rules_actions::program_program_with_main(
                        context,
                        p0,
                        p1,
                        p2,
                        p3,
                        p4,
                        p5,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ProgramProgramOnlyBody => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::Body(p0)) => {
                        NonTerminal::Program(rules_actions::program_program_only_body(
                            context,
                            p0,
                            &mut compiler_context,
                        ))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::BodyBodyInitExpressions => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::TokenInit(p0)),
                        Symbol::NonTerminal(NonTerminal::InitBody(p1)),
                        Symbol::NonTerminal(NonTerminal::Expressions(p2)),
                    ) => NonTerminal::Body(rules_actions::body_body_init_expressions(
                        context,
                        p0,
                        p1,
                        p2,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::BodyBodyInit => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 2usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::TokenInit(p0)),
                        Symbol::NonTerminal(NonTerminal::InitBody(p1)),
                    ) => NonTerminal::Body(rules_actions::body_body_init(
                        context,
                        p0,
                        p1,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::BodyBodyExpressions => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::Expressions(p0)) => NonTerminal::Body(
                        rules_actions::body_body_expressions(context, p0, &mut compiler_context),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::BodyBodyEmpty => NonTerminal::Body(rules_actions::body_body_empty(
                context,
                &mut compiler_context,
            )),
            ProdKind::InitBodyInitBody => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::TokenCBOpen(p0)),
                        Symbol::NonTerminal(NonTerminal::VarDeclarations(p1)),
                        Symbol::Terminal(Terminal::TokenCBClose(p2)),
                    ) => NonTerminal::InitBody(rules_actions::init_body_init_body(
                        context,
                        p0,
                        p1,
                        p2,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::FunctionReadFunctionReadCall => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 4usize)
                    .into_iter();
                match (
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                ) {
                    (
                        Symbol::Terminal(Terminal::TokenRead(p0)),
                        Symbol::Terminal(Terminal::TokenParOpen(p1)),
                        Symbol::Terminal(Terminal::TokenId(p2)),
                        Symbol::Terminal(Terminal::TokenParClose(p3)),
                    ) => {
                        NonTerminal::FunctionRead(rules_actions::function_read_function_read_call(
                            context,
                            p0,
                            p1,
                            p2,
                            p3,
                            &mut compiler_context,
                        ))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::FunctionWriteFunctionWriteCall => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 4usize)
                    .into_iter();
                match (
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                ) {
                    (
                        Symbol::Terminal(Terminal::TokenWrite(p0)),
                        Symbol::Terminal(Terminal::TokenParOpen(p1)),
                        Symbol::NonTerminal(NonTerminal::SimpleExpression(p2)),
                        Symbol::Terminal(Terminal::TokenParClose(p3)),
                    ) => NonTerminal::FunctionWrite(
                        rules_actions::function_write_function_write_call(
                            context,
                            p0,
                            p1,
                            p2,
                            p3,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::FunctionIsZeroFunctionIsZeroCall => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 4usize)
                    .into_iter();
                match (
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                ) {
                    (
                        Symbol::Terminal(Terminal::TokenIsZero(p0)),
                        Symbol::Terminal(Terminal::TokenParOpen(p1)),
                        Symbol::NonTerminal(NonTerminal::ArithmeticExpression(p2)),
                        Symbol::Terminal(Terminal::TokenParClose(p3)),
                    ) => NonTerminal::FunctionIsZero(
                        rules_actions::function_is_zero_function_is_zero_call(
                            context,
                            p0,
                            p1,
                            p2,
                            p3,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::FunctionConvDateFunctionConvDateVariableCall => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 4usize)
                    .into_iter();
                match (
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                ) {
                    (
                        Symbol::Terminal(Terminal::TokenConvDate(p0)),
                        Symbol::Terminal(Terminal::TokenParOpen(p1)),
                        Symbol::Terminal(Terminal::TokenDate(p2)),
                        Symbol::Terminal(Terminal::TokenParClose(p3)),
                    ) => NonTerminal::FunctionConvDate(
                        rules_actions::function_conv_date_function_conv_date_variable_call(
                            context,
                            p0,
                            p1,
                            p2,
                            p3,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::VarDeclarationsVarDeclarationsSingle => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::VarDeclaration(p0)) => {
                        NonTerminal::VarDeclarations(
                            rules_actions::var_declarations_var_declarations_single(
                                context,
                                p0,
                                &mut compiler_context,
                            ),
                        )
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::VarDeclarationsVarDeclarationsRecursive => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 2usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::VarDeclaration(p0)),
                        Symbol::NonTerminal(NonTerminal::VarDeclarations(p1)),
                    ) => NonTerminal::VarDeclarations(
                        rules_actions::var_declarations_var_declarations_recursive(
                            context,
                            p0,
                            p1,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::VarDeclarationVarDeclarationSingle => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::TokenId(p0)),
                        Symbol::Terminal(Terminal::TokenColon(p1)),
                        Symbol::NonTerminal(NonTerminal::DataType(p2)),
                    ) => NonTerminal::VarDeclaration(
                        rules_actions::var_declaration_var_declaration_single(
                            context,
                            p0,
                            p1,
                            p2,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::VarDeclarationVarDeclarationRecursive => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::TokenId(p0)),
                        Symbol::Terminal(Terminal::TokenComma(p1)),
                        Symbol::NonTerminal(NonTerminal::VarDeclaration(p2)),
                    ) => NonTerminal::VarDeclaration(
                        rules_actions::var_declaration_var_declaration_recursive(
                            context,
                            p0,
                            p1,
                            p2,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ExpressionsExpressionSingle => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::Statement(p0)) => {
                        NonTerminal::Expressions(rules_actions::expressions_expression_single(
                            context,
                            p0,
                            &mut compiler_context,
                        ))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ExpressionsExpressionRecursive => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 2usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::Statement(p0)),
                        Symbol::NonTerminal(NonTerminal::Expressions(p1)),
                    ) => NonTerminal::Expressions(rules_actions::expressions_expression_recursive(
                        context,
                        p0,
                        p1,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::StatementStatementAssignment => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::Assignment(p0)) => {
                        NonTerminal::Statement(rules_actions::statement_statement_assignment(
                            context,
                            p0,
                            &mut compiler_context,
                        ))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::StatementStatementIfStatement => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::IfStatement(p0)) => {
                        NonTerminal::Statement(rules_actions::statement_statement_if_statement(
                            context,
                            p0,
                            &mut compiler_context,
                        ))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::StatementStatementElseStatement => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::ElseStatement(p0)) => {
                        NonTerminal::Statement(rules_actions::statement_statement_else_statement(
                            context,
                            p0,
                            &mut compiler_context,
                        ))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::StatementStatementWhile => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::WhileLoop(p0)) => {
                        NonTerminal::Statement(rules_actions::statement_statement_while(
                            context,
                            p0,
                            &mut compiler_context,
                        ))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::StatementStatementWrite => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::FunctionWrite(p0)) => {
                        NonTerminal::Statement(rules_actions::statement_statement_write(
                            context,
                            p0,
                            &mut compiler_context,
                        ))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::StatementStatementRead => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::FunctionRead(p0)) => NonTerminal::Statement(
                        rules_actions::statement_statement_read(context, p0, &mut compiler_context),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::AssignmentAssignmentExpression => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::TokenId(p0)),
                        Symbol::Terminal(Terminal::TokenAssign(p1)),
                        Symbol::NonTerminal(NonTerminal::SimpleExpression(p2)),
                    ) => NonTerminal::Assignment(rules_actions::assignment_assignment_expression(
                        context,
                        p0,
                        p1,
                        p2,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::AssignmentAssignmentConvDate => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::TokenId(p0)),
                        Symbol::Terminal(Terminal::TokenAssign(p1)),
                        Symbol::NonTerminal(NonTerminal::FunctionConvDate(p2)),
                    ) => NonTerminal::Assignment(rules_actions::assignment_assignment_conv_date(
                        context,
                        p0,
                        p1,
                        p2,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::DataTypeIntType => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenInt(p0)) => NonTerminal::DataType(
                        rules_actions::data_type_int_type(context, p0, &mut compiler_context),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::DataTypeFloatType => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenFloat(p0)) => NonTerminal::DataType(
                        rules_actions::data_type_float_type(context, p0, &mut compiler_context),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::DataTypeStringType => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenString(p0)) => NonTerminal::DataType(
                        rules_actions::data_type_string_type(context, p0, &mut compiler_context),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::WhileLoopWhile => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 7usize)
                    .into_iter();
                match (
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                ) {
                    (
                        Symbol::Terminal(Terminal::TokenWhile(p0)),
                        Symbol::Terminal(Terminal::TokenParOpen(p1)),
                        Symbol::NonTerminal(NonTerminal::BooleanExpression(p2)),
                        Symbol::Terminal(Terminal::TokenParClose(p3)),
                        Symbol::Terminal(Terminal::TokenCBOpen(p4)),
                        Symbol::NonTerminal(NonTerminal::Body(p5)),
                        Symbol::Terminal(Terminal::TokenCBClose(p6)),
                    ) => NonTerminal::WhileLoop(rules_actions::while_loop_while(
                        context,
                        p0,
                        p1,
                        p2,
                        p3,
                        p4,
                        p5,
                        p6,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::IfStatementIfStatement => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 7usize)
                    .into_iter();
                match (
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                ) {
                    (
                        Symbol::Terminal(Terminal::TokenIf(p0)),
                        Symbol::Terminal(Terminal::TokenParOpen(p1)),
                        Symbol::NonTerminal(NonTerminal::BooleanExpression(p2)),
                        Symbol::Terminal(Terminal::TokenParClose(p3)),
                        Symbol::Terminal(Terminal::TokenCBOpen(p4)),
                        Symbol::NonTerminal(NonTerminal::Body(p5)),
                        Symbol::Terminal(Terminal::TokenCBClose(p6)),
                    ) => NonTerminal::IfStatement(rules_actions::if_statement_if_statement(
                        context,
                        p0,
                        p1,
                        p2,
                        p3,
                        p4,
                        p5,
                        p6,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ElseStatementElseStatement => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 4usize)
                    .into_iter();
                match (
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                ) {
                    (
                        Symbol::Terminal(Terminal::TokenElse(p0)),
                        Symbol::Terminal(Terminal::TokenCBOpen(p1)),
                        Symbol::NonTerminal(NonTerminal::Body(p2)),
                        Symbol::Terminal(Terminal::TokenCBClose(p3)),
                    ) => NonTerminal::ElseStatement(rules_actions::else_statement_else_statement(
                        context,
                        p0,
                        p1,
                        p2,
                        p3,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::BooleanExpressionBooleanExpressionSimpleExpression => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 2usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::SimpleExpression(p0)),
                        Symbol::NonTerminal(NonTerminal::BooleanExpressionChain(p1)),
                    ) => NonTerminal::BooleanExpression(
                        rules_actions::boolean_expression_boolean_expression_simple_expression(
                            context,
                            p0,
                            p1,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::BooleanExpressionBooleanExpressionTrue => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenTrue(p0)) => NonTerminal::BooleanExpression(
                        rules_actions::boolean_expression_boolean_expression_true(
                            context,
                            p0,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::BooleanExpressionBooleanExpressionFalse => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenFalse(p0)) => NonTerminal::BooleanExpression(
                        rules_actions::boolean_expression_boolean_expression_false(
                            context,
                            p0,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::BooleanExpressionBooleanExpressionSimpleExpressionRecursive => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 4usize)
                    .into_iter();
                match (
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                    i.next().unwrap(),
                ) {
                    (
                        Symbol::NonTerminal(NonTerminal::SimpleExpression(p0)),
                        Symbol::NonTerminal(NonTerminal::BooleanExpressionChain(p1)),
                        Symbol::NonTerminal(NonTerminal::Conjunction(p2)),
                        Symbol::NonTerminal(NonTerminal::BooleanExpression(p3)),
                    ) => {
                        NonTerminal::BooleanExpression(
                            rules_actions::boolean_expression_boolean_expression_simple_expression_recursive(
                                context,
                                p0,
                                p1,
                                p2,
                                p3,&mut compiler_context
                            ),
                        )
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::BooleanExpressionBooleanExpressionNotStatement => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::NotStatement(p0)) => {
                        NonTerminal::BooleanExpression(
                            rules_actions::boolean_expression_boolean_expression_not_statement(
                                context,
                                p0,
                                &mut compiler_context,
                            ),
                        )
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::BooleanExpressionBooleanExpressionIsZero => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::FunctionIsZero(p0)) => {
                        NonTerminal::BooleanExpression(
                            rules_actions::boolean_expression_boolean_expression_is_zero(
                                context,
                                p0,
                                &mut compiler_context,
                            ),
                        )
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::BooleanExpressionChainBooleanExpressionChainAux => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::ComparisonOp(p0)),
                        Symbol::NonTerminal(NonTerminal::SimpleExpression(p1)),
                        Symbol::NonTerminal(NonTerminal::BooleanExpressionChain(p2)),
                    ) => NonTerminal::BooleanExpressionChain(
                        rules_actions::boolean_expression_chain_boolean_expression_chain_aux(
                            context,
                            p0,
                            p1,
                            p2,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::BooleanExpressionChainBooleanExpressionChainEmpty => {
                NonTerminal::BooleanExpressionChain(
                    rules_actions::boolean_expression_chain_boolean_expression_chain_empty(
                        context,
                        &mut compiler_context,
                    ),
                )
            }
            ProdKind::SimpleExpressionSimpleExpressionArithmetic => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::ArithmeticExpression(p0)) => {
                        NonTerminal::SimpleExpression(
                            rules_actions::simple_expression_simple_expression_arithmetic(
                                context,
                                p0,
                                &mut compiler_context,
                            ),
                        )
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::SimpleExpressionSimpleExpressionString => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenStringLiteral(p0)) => {
                        NonTerminal::SimpleExpression(
                            rules_actions::simple_expression_simple_expression_string(
                                context,
                                p0,
                                &mut compiler_context,
                            ),
                        )
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ConjunctionConjunctionAnd => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenAnd(p0)) => {
                        NonTerminal::Conjunction(rules_actions::conjunction_conjunction_and(
                            context,
                            p0,
                            &mut compiler_context,
                        ))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ConjunctionConjunctionOr => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenOr(p0)) => {
                        NonTerminal::Conjunction(rules_actions::conjunction_conjunction_or(
                            context,
                            p0,
                            &mut compiler_context,
                        ))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ComparisonOpComparisonOpEqual => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenEqual(p0)) => {
                        NonTerminal::ComparisonOp(rules_actions::comparison_op_comparison_op_equal(
                            context,
                            p0,
                            &mut compiler_context,
                        ))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ComparisonOpComparisonOpNotEqual => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenNotEqual(p0)) => NonTerminal::ComparisonOp(
                        rules_actions::comparison_op_comparison_op_not_equal(
                            context,
                            p0,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ComparisonOpComparisonOpLess => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenLess(p0)) => {
                        NonTerminal::ComparisonOp(rules_actions::comparison_op_comparison_op_less(
                            context,
                            p0,
                            &mut compiler_context,
                        ))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ComparisonOpComparisonOpLessEqual => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenLessEqual(p0)) => NonTerminal::ComparisonOp(
                        rules_actions::comparison_op_comparison_op_less_equal(
                            context,
                            p0,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ComparisonOpComparisonOpGreater => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenGreater(p0)) => NonTerminal::ComparisonOp(
                        rules_actions::comparison_op_comparison_op_greater(
                            context,
                            p0,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ComparisonOpComparisonOpGreaterEqual => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenGreaterEqual(p0)) => NonTerminal::ComparisonOp(
                        rules_actions::comparison_op_comparison_op_greater_equal(
                            context,
                            p0,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::NumberNumberInt => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenIntLiteral(p0)) => NonTerminal::Number(
                        rules_actions::number_number_int(context, p0, &mut compiler_context),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::NumberNumberFloat => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenFloatLiteral(p0)) => NonTerminal::Number(
                        rules_actions::number_number_float(context, p0, &mut compiler_context),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::NumberNumberNegativeInt => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 2usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::TokenSub(p0)),
                        Symbol::Terminal(Terminal::TokenIntLiteral(p1)),
                    ) => NonTerminal::Number(rules_actions::number_number_negative_int(
                        context,
                        p0,
                        p1,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::NumberNumberNegativeFloat => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 2usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::TokenSub(p0)),
                        Symbol::Terminal(Terminal::TokenFloatLiteral(p1)),
                    ) => NonTerminal::Number(rules_actions::number_number_negative_float(
                        context,
                        p0,
                        p1,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::NotStatementNot => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 2usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::TokenNot(p0)),
                        Symbol::NonTerminal(NonTerminal::BooleanExpression(p1)),
                    ) => NonTerminal::NotStatement(rules_actions::not_statement_not(
                        context,
                        p0,
                        p1,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ArithmeticExpressionArithmeticExpressionSumTerm => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::ArithmeticExpression(p0)),
                        Symbol::Terminal(Terminal::TokenSum(p1)),
                        Symbol::NonTerminal(NonTerminal::Term(p2)),
                    ) => NonTerminal::ArithmeticExpression(
                        rules_actions::arithmetic_expression_arithmetic_expression_sum_term(
                            context,
                            p0,
                            p1,
                            p2,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ArithmeticExpressionArithmeticExpressionSubTerm => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::ArithmeticExpression(p0)),
                        Symbol::Terminal(Terminal::TokenSub(p1)),
                        Symbol::NonTerminal(NonTerminal::Term(p2)),
                    ) => NonTerminal::ArithmeticExpression(
                        rules_actions::arithmetic_expression_arithmetic_expression_sub_term(
                            context,
                            p0,
                            p1,
                            p2,
                            &mut compiler_context,
                        ),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::ArithmeticExpressionArithmeticExpressionTerm => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::Term(p0)) => {
                        NonTerminal::ArithmeticExpression(
                            rules_actions::arithmetic_expression_arithmetic_expression_term(
                                context,
                                p0,
                                &mut compiler_context,
                            ),
                        )
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::TermTermMulFactor => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::Term(p0)),
                        Symbol::Terminal(Terminal::TokenMul(p1)),
                        Symbol::NonTerminal(NonTerminal::Factor(p2)),
                    ) => NonTerminal::Term(rules_actions::term_term_mul_factor(
                        context,
                        p0,
                        p1,
                        p2,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::TermTermDivFactor => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::Term(p0)),
                        Symbol::Terminal(Terminal::TokenDiv(p1)),
                        Symbol::NonTerminal(NonTerminal::Factor(p2)),
                    ) => NonTerminal::Term(rules_actions::term_term_div_factor(
                        context,
                        p0,
                        p1,
                        p2,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::TermTermFactor => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::Factor(p0)) => NonTerminal::Term(
                        rules_actions::term_term_factor(context, p0, &mut compiler_context),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::FactorFactorId => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::TokenId(p0)) => NonTerminal::Factor(
                        rules_actions::factor_factor_id(context, p0, &mut compiler_context),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::FactorFactorNumber => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::Number(p0)) => NonTerminal::Factor(
                        rules_actions::factor_factor_number(context, p0, &mut compiler_context),
                    ),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::FactorFactorParen => {
                let mut i = compiler_context
                    .res_stack
                    .split_off(stack_len - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::TokenParOpen(p0)),
                        Symbol::NonTerminal(NonTerminal::ArithmeticExpression(p1)),
                        Symbol::Terminal(Terminal::TokenParClose(p2)),
                    ) => NonTerminal::Factor(rules_actions::factor_factor_paren(
                        context,
                        p0,
                        p1,
                        p2,
                        &mut compiler_context,
                    )),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
        };
        compiler_context.res_stack.push(Symbol::NonTerminal(prod));
    }
}
