use super::rules::{Context, TokenKind};
use super::rules_lexer::Input;
use crate::compiler::context::{CompilerContext, SymbolTableElement};
use crate::compiler::error::{CompilerError, log_error_and_exit};
use rustemo::Token as RustemoToken;
use std::fmt::Display;

pub type Ctx<'i> = Context<'i, Input>;
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;

/// Keyword "int"
pub type TokenInt = String;

/// Keyword "float"
pub type TokenFloat = String;

/// Keyword "string"
pub type TokenString = String;

/// Signed integer number
pub type TokenIntLiteral = i64;

/// Signed float with 32 bits precision
#[derive(Debug, Clone)]
pub struct TokenFloatLiteral {
    /// Original string representing the float
    pub original: String,
    /// Parsed representation of the float
    pub parsed: f32,
}

impl PartialEq for TokenFloatLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.original == other.original
    }
}

/// Parsed string literal without ""
pub type TokenStringLiteral = String;

/// Token representing an identifier
pub type TokenId = String;

/// Keyword ":="
pub type TokenAssign = String;

/// Keyword "+"
pub type TokenSum = String;

/// Keyword "*"
pub type TokenMul = String;

/// Keyword "-"
pub type TokenSub = String;

/// Keyword "/"
pub type TokenDiv = String;

/// Keyword "("
pub type TokenParOpen = String;

/// Keyword ")"
pub type TokenParClose = String;

/// Keyword "{"
pub type TokenCBOpen = String;

/// Keyword "}"
pub type TokenCBClose = String;

/// Keyword ":"
pub type TokenColon = String;

/// Keyword "init"
pub type TokenInit = String;

/// Keyword "while"
pub type TokenWhile = String;

/// Keyword "=="
pub type TokenEqual = String;

/// Keyword "!="
pub type TokenNotEqual = String;

/// Keyword "<"
pub type TokenLess = String;

/// Keyword "<="
pub type TokenLessEqual = String;

/// Keyword ">"
pub type TokenGreater = String;

/// Keyword ">="
pub type TokenGreaterEqual = String;

/// Keyword "true"
pub type TokenTrue = String;

/// Keyword "False"
pub type TokenFalse = String;

/// Keyword "if"
pub type TokenIf = String;

/// Keyword "else"
pub type TokenElse = String;

/// Keyword ","
pub type TokenComma = String;

/// Keyword "and"
pub type TokenAnd = String;

/// Keyword "or"
pub type TokenOr = String;

/// Keyword "not"
pub type TokenNot = String;

/// Keyword "read"
pub type TokenRead = String;

/// Keyword "write"
pub type TokenWrite = String;

/// Keyword "isZero"
pub type TokenIsZero = String;

/// Keyword "convDate"
pub type TokenConvDate = String;

/// Date in "DD-MM-YYYY" format
#[derive(Debug, Clone)]
pub struct TokenDate {
    pub day: String,
    pub month: String,
    pub year: String,
}

impl Display for TokenDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.day, self.month, self.year)
    }
}

/// Enum representing all the possible rules for the `<Program>` non terminal
#[derive(Debug, Clone)]
pub enum Program {
    /// `<Program> -> TokenId TokenParOpen TokenParClose TokenCBOpen <Body> TokenCBClose`
    ProgramWithMain(ProgramWithMain),
    /// `<Program> -> <Body>`
    ProgramOnlyBody(Body),
}

/// Struct representation of the rule
///
/// `<Program> -> TokenId TokenParOpen TokenParClose TokenCBOpen <Body> TokenCBClose`
#[derive(Debug, Clone)]
pub struct ProgramWithMain {
    pub token_id: TokenId,
    pub token_par_open: TokenParOpen,
    pub token_par_close: TokenParClose,
    pub token_cbopen: TokenCBOpen,
    pub body: Body,
    pub token_cbclose: TokenCBClose,
}

/// Enum representing all the possible rules for the `<Body>` non terminal
#[derive(Debug, Clone)]
pub enum BodyNoO {
    /// `<Body> -> TokenInit <InitBody> <Expressions>`
    BodyInitExpressions(BodyInitExpressions),
    /// `<Body> -> TokenInit <InitBody>`
    BodyInit(BodyInit),
    /// `<Body> -> <Expressions>`
    BodyExpressions(Expressions),
}

/// Type declaration for the `<Body>` non terminal represented by an `Option<T>` because it can be empty
pub type Body = Option<BodyNoO>;

/// Struct representation of the rule
///
/// `<Body> -> TokenInit <InitBody> <Expression>`
#[derive(Debug, Clone)]
pub struct BodyInitExpressions {
    pub token_init: TokenInit,
    pub init_body: InitBody,
    pub expressions: Expressions,
}

/// Struct representation of the rule
///
/// `<Body> -> TokenInit <InitBody>`
#[derive(Debug, Clone)]
pub struct BodyInit {
    pub token_init: TokenInit,
    pub init_body: InitBody,
}

/// Struct representation of the rule
///
/// `<Body> -> TokenCBOpen <VarDeclarations> TokenCBClose`
#[derive(Debug, Clone)]
pub struct InitBody {
    pub token_cbopen: TokenCBOpen,
    pub var_declarations: VarDeclarations,
    pub token_cbclose: TokenCBClose,
}

/// Struct representation of the rule
///
/// `<FunctionRead> -> TokenRead TokenParOpen TokenId TokenParClose`
#[derive(Debug, Clone)]
pub struct FunctionRead {
    pub token_read: TokenRead,
    pub token_par_open: TokenParOpen,
    pub token_id: TokenId,
    pub token_par_close: TokenParClose,
}

/// Struct representation of the rule
///
/// `<FunctionWrite> -> TokenWrite TokenParOpen <SimpleExpression> TokenParClose`
#[derive(Debug, Clone)]
pub struct FunctionWrite {
    pub token_write: TokenWrite,
    pub token_par_open: TokenParOpen,
    pub simple_expression: SimpleExpression,
    pub token_par_close: TokenParClose,
}

/// Struct representation of the rule
///
/// `<FunctionIsZero> -> TokenIsZero TokenParOpen <ArithmeticExpression> TokenParClose`
#[derive(Debug, Clone)]
pub struct FunctionIsZero {
    pub token_is_zero: TokenIsZero,
    pub token_par_open: TokenParOpen,
    pub arithmetic_expression: ArithmeticExpression,
    pub token_par_close: TokenParClose,
}

/// Struct representation of the rule
///
/// `<FunctionConvDate> -> TokenConvDate TokenParOpen TokenDate TokenParClose`
#[derive(Debug, Clone)]
pub struct FunctionConvDate {
    pub token_conv_date: TokenConvDate,
    pub token_par_open: TokenParOpen,
    pub token_date: TokenDate,
    pub token_par_close: TokenParClose,
}

/// Enum representing all the possible rules for the `<VarDeclarations>` non terminal
#[derive(Debug, Clone)]
pub enum VarDeclarations {
    /// `<VarDeclarations> -> <VarDeclaration>`
    VarDeclarationsSingle(VarDeclaration),
    /// `<VarDeclarations> -> <VarDeclaration> <VarDeclarations>`
    VarDeclarationsRecursive(VarDeclarationsRecursive),
}

/// Struct representation of the rule
///
/// `<VarDeclarations> -> <VarDeclaration> <VarDeclarations>`
#[derive(Debug, Clone)]
pub struct VarDeclarationsRecursive {
    pub var_declaration: VarDeclaration,
    pub var_declarations: Box<VarDeclarations>,
}

/// Enum representing all the possible rules for the `<VarDeclaration>` non terminal
#[derive(Debug, Clone)]
pub enum VarDeclaration {
    /// `<VarDeclaration> -> TokenId TokenColon <DataType>`
    VarDeclarationSingle(VarDeclarationSingle),
    /// `<VarDeclaration> -> TokenId TokenComma <VarDeclaration>`
    VarDeclarationRecursive(VarDeclarationRecursive),
}

/// Struct representation of the rule
///
/// `<VarDeclaration> -> TokenId TokenComma <VarDeclaration>`
#[derive(Debug, Clone)]
pub struct VarDeclarationRecursive {
    pub token_id: TokenId,
    pub token_comma: TokenComma,
    pub var_declaration: Box<VarDeclaration>,
}

/// Struct representation of the rule
///
/// `<VarDeclaration> -> TokenId TokenColon <DataType>`
#[derive(Debug, Clone)]
pub struct VarDeclarationSingle {
    pub token_id: TokenId,
    pub token_colon: TokenColon,
    pub data_type: DataType,
}

impl VarDeclaration {
    /// Recursively traverse `VarDeclaration` until we find the non recurive declaration with the DataType
    /// During traversal, each variable is added to the symbol table
    /// If a symbol already exists then we error out with a variable redeclaration error
    pub fn push_to_symbol_table(&self, compiler_context: &mut CompilerContext) -> DataType {
        match self {
            Self::VarDeclarationSingle(single) => {
                let symbol = SymbolTableElement::VarDeclaration(
                    single.token_id.clone(),
                    single.data_type.clone(),
                    single.token_id.len(),
                );
                // If the symbol already exists this is a redeclaration
                if compiler_context.symbol_exists(&symbol) {
                    log_error_and_exit(
                        0..0,
                        CompilerError::Parser(format!(
                            "Redeclaration of variable {}",
                            single.token_id
                        )),
                        0,
                        false,
                        compiler_context,
                    );
                } else {
                    compiler_context.push_to_symbol_table(symbol);
                }
                single.data_type.clone()
            }
            Self::VarDeclarationRecursive(recursive) => {
                let data_type = recursive
                    .var_declaration
                    .push_to_symbol_table(compiler_context);
                let symbol = SymbolTableElement::VarDeclaration(
                    recursive.token_id.clone(),
                    data_type.clone(),
                    recursive.token_id.len(),
                );
                // If the symbol already exists this is a redeclaration
                if compiler_context.symbol_exists(&symbol) {
                    log_error_and_exit(
                        0..0,
                        CompilerError::Parser(format!(
                            "Redeclaration of variable {}",
                            recursive.token_id
                        )),
                        0,
                        false,
                        compiler_context,
                    )
                } else {
                    compiler_context.push_to_symbol_table(symbol);
                }
                data_type
            }
        }
    }
}

/// Enum representing all the possible rules for the `<Expressions>` non terminal
#[derive(Debug, Clone)]
pub enum Expressions {
    /// `<Expressions> -> <Statement>`
    ExpressionSingle(Statement),
    /// `<Expressions> -> <Statement> <Expressions>`
    ExpressionRecursive(ExpressionRecursive),
}

/// Struct representation of the rule
///
/// `<Expressions> -> <Statement> <Expressions>`
#[derive(Debug, Clone)]
pub struct ExpressionRecursive {
    pub statement: Statement,
    pub expressions: Box<Expressions>,
}

/// Enum representing all the possible rules for the `<Statement>` non terminal
#[derive(Debug, Clone)]
pub enum Statement {
    /// `<Statement> -> <Assignment>`
    StatementAssignment(Assignment),
    /// `<Statement> -> <IfStatement>`
    StatementIfStatement(IfStatement),
    /// `<Statement> -> <ElseStatement>`
    StatementElseStatement(ElseStatement),
    /// `<Statement> -> <WhileLoop>`
    StatementWhile(WhileLoop),
    /// `<Statement> -> <FunctionWrite>`
    StatementWrite(FunctionWrite),
    /// `<Statement> -> <FunctionRead>`
    StatementRead(FunctionRead),
}

/// Enum representing all the possible rules for the `<Assignment>` non terminal
#[derive(Debug, Clone)]
pub enum Assignment {
    /// `<Assignment> -> TokenId TokenAssign <SimpleExpression>`
    AssignmentExpression(AssignmentExpression),
    /// `<Assignment> -> TokenId TokenAssign <FunctionConvDate>`
    AssignmentConvDate(ConvDate),
}

/// Struct representation of the rule
///
/// `<Assignment> -> TokenId TokenAssign <SimpleExpression>`
#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    pub token_id: TokenId,
    pub token_assign: TokenAssign,
    pub simple_expression: SimpleExpression,
}

/// Struct representation of the rule
///
/// `<Assignment> -> TokenId TokenAssign <FunctionConvDate>`
#[derive(Debug, Clone)]
pub struct ConvDate {
    pub token_id: TokenId,
    pub token_assign: TokenAssign,
    pub function_conv_date: FunctionConvDate,
}

/// Enum representing all the possible rules for the `<DataType>` non terminal
#[derive(Debug, Clone)]
pub enum DataType {
    /// `<DataType> -> "int"`
    IntType(TokenInt),
    /// `<DataType> -> "float"`
    FloatType(TokenFloat),
    /// `<DataType> -> "string"`
    StringType(TokenString),
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DataType::IntType(_) => "VAR_INT",
            DataType::FloatType(_) => "VAR_FLOAT",
            DataType::StringType(_) => "VAR_STRING",
        };
        write!(f, "{s}")
    }
}

/// Struct representation of the rule
///
/// `<WhileLoop> -> TokenWhile TokenParOpen <BooleanExpression> TokenParClose TokenCBOpen <Body> TokenCBClose`
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

/// Struct representation of the rule
///
/// `<IfStatement> -> TokenIf TokenParOpen <BooleanExpression> TokenParClose TokenCBOpen <Body> TokenCBClose`
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

/// Struct representation of the rule
///
/// `<ElseStatement> -> TokenElse TokenCBOpen <Body> TokenCBClose`
#[derive(Debug, Clone)]
pub struct ElseStatement {
    pub token_else: TokenElse,
    pub token_cbopen: TokenCBOpen,
    pub body: Box<Body>,
    pub token_cbclose: TokenCBClose,
}

/// Enum representing all the possible rules for the `<BooleanExpression>` non terminal
#[derive(Debug, Clone)]
pub enum BooleanExpression {
    /// `<BooleanExpression> -> <SimpleExpression> <BooleanExpressionChain>`
    BooleanExpressionSimpleExpression(BooleanExpressionSimpleExpression),
    /// `<BooleanExpression> -> "true"`
    BooleanExpressionTrue(TokenTrue),
    /// `<BooleanExpression> -> "false"`
    BooleanExpressionFalse(TokenFalse),
    /// `<BooleanExpression> -> <SimpleExpression> <BooleanExpressionChain> <Conjunction> <BooleanExpression>`
    BooleanExpressionSimpleExpressionRecursive(BooleanExpressionSimpleExpressionRecursive),
    /// `<BooleanExpression> -> <NotStatement>`
    BooleanExpressionNotStatement(NotStatement),
    /// `<BooleanExpression> -> <FunctionIsZero>`
    BooleanExpressionIsZero(FunctionIsZero),
}

/// Struct representation of the rule
///
/// `<BooleanExpression> -> <SimpleExpression> <BooleanExpressionChain>`
#[derive(Debug, Clone)]
pub struct BooleanExpressionSimpleExpression {
    pub simple_expression: SimpleExpression,
    pub boolean_expression_chain: BooleanExpressionChain,
}

/// Struct representation of the rule
///
/// `<BooleanExpression> -> <SimpleExpression> <BooleanExpressionChain> <Conjunction> <BooleanExpression>`
#[derive(Debug, Clone)]
pub struct BooleanExpressionSimpleExpressionRecursive {
    pub simple_expression: SimpleExpression,
    pub boolean_expression_chain: BooleanExpressionChain,
    pub conjunction: Conjunction,
    pub boolean_expression: Box<BooleanExpression>,
}

/// Type declaration for the `<BooleanExpressionChain>` non terminal represented by an `Option<T>` because it can be empty
pub type BooleanExpressionChain = Option<BooleanExpressionChainNoO>;

/// Struct representation of the rule
///
/// `<BooleanExpressionChain> -> ComparisonOp <SimpleExpression> <BooleanExpressionChain>`
#[derive(Debug, Clone)]
pub struct BooleanExpressionChainNoO {
    pub comparison_op: ComparisonOp,
    pub simple_expression: SimpleExpression,
    pub boolean_expression_chain: Box<BooleanExpressionChain>,
}

/// Enum representing all the possible rules for the `<SimpleExpression>` non terminal
#[derive(Debug, Clone)]
pub enum SimpleExpression {
    /// `<SimpleExpression> -> <ArithmeticExpression>`
    SimpleExpressionArithmeticExpression(ArithmeticExpression),
    /// `<SimpleExpression> -> TokenStringLiteral`
    SimpleExpressionString(TokenStringLiteral),
}

/// Enum representing all the possible rules for the `<Conjunction>` non terminal
#[derive(Debug, Clone)]
pub enum Conjunction {
    /// `<Conjunction> -> "and"`
    ConjunctionAnd(TokenAnd),
    /// `<Conjunction> -> "or"`
    ConjunctionOr(TokenOr),
}

/// Enum representing all the possible rules for the `<ComparisonOp>` non terminal
#[derive(Debug, Clone)]
pub enum ComparisonOp {
    /// `<ComparisonOp> -> "=="`
    ComparisonOpEqual(TokenEqual),
    /// `<ComparisonOp> -> "!="`
    ComparisonOpNotEqual(TokenNotEqual),
    /// `<ComparisonOp> -> "<"`
    ComparisonOpLess(TokenLess),
    /// `<ComparisonOp> -> "<="`
    ComparisonOpLessEqual(TokenLessEqual),
    /// `<ComparisonOp> -> ">"`
    ComparisonOpGreater(TokenGreater),
    /// `<ComparisonOp> -> ">="`
    ComparisonOpGreaterEqual(TokenGreaterEqual),
}

/// Enum representing all the possible rules for the `<Number>` non terminal
#[derive(Debug, Clone)]
pub enum Number {
    /// `<Number> -> TokenIntLiteral`
    NumberInt(TokenIntLiteral),
    /// `<Number> -> TokenFloatLiteral`
    NumberFloat(TokenFloatLiteral),
}

/// Struct representation of the rule
///
/// `<NotStatement> -> TokenNot <BooleanExpression>`
#[derive(Debug, Clone)]
pub struct NotStatement {
    pub token_not: TokenNot,
    pub boolean_expression: Box<BooleanExpression>,
}

/// Enum representing all the possible rules for the `<ArithmeticExpression>` non terminal
#[derive(Debug, Clone)]
pub enum ArithmeticExpression {
    /// `<ArithmeticExpression> -> <ArithmeticExpression> TokenSum <Term>`
    ArithmeticExpressionSumTerm(ArithmeticExpressionSumTerm),
    /// `<ArithmeticExpression> -> <ArithmeticExpression> TokenSub <Term>`
    ArithmeticExpressionSubTerm(ArithmeticExpressionSubTerm),
    /// `<ArithmeticExpression> -> <Term>`
    ArithmeticExpressionTerm(Term),
}

/// Struct representation of the rule
///
/// `<ArithmeticExpression> -> <ArithmeticExpression> TokenSum <Term>`
#[derive(Debug, Clone)]
pub struct ArithmeticExpressionSumTerm {
    pub arithmetic_expression: Box<ArithmeticExpression>,
    pub token_sum: TokenSum,
    pub term: Term,
}

/// Struct representation of the rule
///
/// `<ArithmeticExpression> -> <ArithmeticExpression> TokenSub <Term>`
#[derive(Debug, Clone)]
pub struct ArithmeticExpressionSubTerm {
    pub arithmetic_expression: Box<ArithmeticExpression>,
    pub token_sub: TokenSub,
    pub term: Term,
}

/// Enum representing all the possible rules for the `<Term>` non terminal
#[derive(Debug, Clone)]
pub enum Term {
    /// `<Term> -> <Term> TokenMul <Factor>`
    TermMulFactor(TermMulFactor),
    /// `<Term> -> <Term> TokenDiv <Factor>`
    TermDivFactor(TermDivFactor),
    /// `<Term> -> <Factor>`
    TermFactor(Factor),
}

/// Struct representation of the rule
///
/// `<Term> -> <Term> TokenMul <Factor>`
#[derive(Debug, Clone)]
pub struct TermMulFactor {
    pub term: Box<Term>,
    pub token_mul: TokenMul,
    pub factor: Factor,
}

/// Struct representation of the rule
///
/// `<Term> -> <Term> TokenDiv <Factor>`
#[derive(Debug, Clone)]
pub struct TermDivFactor {
    pub term: Box<Term>,
    pub token_div: TokenDiv,
    pub factor: Factor,
}

/// Enum representing all the possible rules for the `<Factor>` non terminal
#[derive(Debug, Clone)]
pub enum Factor {
    /// `<Factor> -> TokenId`
    FactorId(TokenId),
    /// `<Factor> -> <Number>`
    FactorNumber(Number),
    /// `<Factor> -> TokenParOpen <ArithmeticExpression> TokenParClose`
    FactorParen(FactorParen),
}

/// Struct representation of the rule
///
/// `<Factor> -> TokenParOpen <ArithmeticExpression> TokenParClose`
#[derive(Debug, Clone)]
pub struct FactorParen {
    pub token_par_open: TokenParOpen,
    pub arithmetic_expression: Box<ArithmeticExpression>,
    pub token_par_close: TokenParClose,
}

/// Enum used for all the non terminals used as markers for generating the intermediate representation
#[derive(Debug)]
pub enum Dummy {}

/// Type declaration for the `<DummyAE>` non terminal represented by an `Option<T>` because it should always be EMPTY
pub type DummyAE = Option<Dummy>;

/// Type declaration for the `<DummyAE>` non terminal represented by an `Option<T>` because it should always be EMPTY
pub type DummyT = Option<Dummy>;

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::STOP => "EOF",
            Self::TokenInt => "\"int\"",
            Self::TokenFloat => "\"float\"",
            Self::TokenString => "\"string\"",
            Self::TokenIntLiteral => "\"integer literal\"",
            Self::TokenFloatLiteral => "\"float literal\"",
            Self::TokenStringLiteral => "\"string literal\"",
            Self::TokenId => "\"identifier\"",
            Self::TokenAssign => "\":=\"",
            Self::TokenSum => "\"+\"",
            Self::TokenMul => "\"*\"",
            Self::TokenSub => "\"-\"",
            Self::TokenDiv => "\"/\"",
            Self::TokenParOpen => "\"(\"",
            Self::TokenParClose => "\")\"",
            Self::TokenCBOpen => "\"{\"",
            Self::TokenCBClose => "\"}\"",
            Self::TokenColon => "\":\"",
            Self::TokenInit => "\"init\"",
            Self::TokenWhile => "\"while\"",
            Self::TokenEqual => "\"==\"",
            Self::TokenNotEqual => "\"!=\"",
            Self::TokenLess => "\"<\"",
            Self::TokenLessEqual => "\"<=\"",
            Self::TokenGreater => "\">\"",
            Self::TokenGreaterEqual => "\">=\"",
            Self::TokenTrue => "\"true\"",
            Self::TokenFalse => "\"false\"",
            Self::TokenIf => "\"if\"",
            Self::TokenElse => "\"else\"",
            Self::TokenComma => "\",\"",
            Self::TokenAnd => "\"and\"",
            Self::TokenOr => "\"or\"",
            Self::TokenNot => "\"not\"",
            Self::TokenRead => "\"read\"",
            Self::TokenWrite => "\"write\"",
            Self::TokenIsZero => "\"isZero\"",
            Self::TokenConvDate => "\"convDate\"",
            Self::TokenDate => "\"date\"",
        };
        write!(f, "{text}")
    }
}
