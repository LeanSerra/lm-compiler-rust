use crate::{
    CompilerError,
    context::SOURCE_CODE_PATH,
    grammar::{State, TokenKind},
};

use owo_colors::OwoColorize;
use rustemo::{Context, LRContext, Lexer, Location, Position, Token};
use std::{fmt::Display, iter, ops::Range};

/// We are parsing a slice of bytes.
pub type Input = str;
pub type Ctx<'i> = LRContext<'i, Input, State, TokenKind>;

#[derive(Default)]
pub struct LexerAdapter();

impl LexerAdapter {
    pub fn new() -> LexerAdapter {
        Self()
    }
}

impl<'i> Lexer<'i, Ctx<'i>, State, TokenKind> for LexerAdapter {
    type Input = Input;

    fn next_tokens(
        &self,
        context: &mut Ctx<'i>,
        input: &'i Self::Input,
        expected_tokens: Vec<(TokenKind, bool)>,
    ) -> Box<dyn Iterator<Item = Token<'i, Self::Input, TokenKind>> + 'i> {
        let expected_tokens = expected_tokens
            .into_iter()
            .map(|tok| tok.0)
            .collect::<Vec<_>>();
        let expected_tokens_str = expected_tokens
            .iter()
            .map(|tok| tok.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let mut pos = context.position();

        let value;
        let token;

        if context.position() >= input.len() || input.is_empty() {
            value = "";
            token = TokenKind::STOP
        } else {
            let trimmed_input = input.get(context.position()..input.len()).unwrap();
            let mut lexer = crate::lex::Lexer::new(trimmed_input, pos);
            token =
                match validate_and_get_next_token(&mut lexer, expected_tokens, expected_tokens_str)
                {
                    Ok(tok) => tok,
                    Err(e) => {
                        log_error(lexer.yytextpos(), e, pos, input, true);
                        std::process::exit(1)
                    }
                };
            let range = lexer.yytextpos();
            pos += range.start;
            value = unsafe { trimmed_input.get_unchecked(range) };
        }

        context.set_position(pos);

        Box::new(iter::once(Token {
            kind: token,
            value,
            location: Location {
                start: Position::Position(context.position()),
                end: Some(Position::Position(pos)),
            },
        }))
    }
}

fn validate_and_get_next_token(
    lexer: &mut crate::lex::Lexer,
    expected_tokens: Vec<TokenKind>,
    expected_tokens_str: String,
) -> Result<TokenKind, CompilerError> {
    match lexer.yylex() {
        Ok(token) => {
            if !expected_tokens.contains(&token) {
                Err(CompilerError::Parser(format!(
                    "unexpected token: {token} expected one of: {expected_tokens_str}"
                )))
            } else {
                Ok(token)
            }
        }
        Err(e) => {
            if let crate::lex::Error::Unmatch = e {
                Err(CompilerError::Lexer(format!(
                    "unrecognized token {}",
                    lexer.yytext()
                )))
            } else if !expected_tokens.contains(&TokenKind::STOP) {
                Err(CompilerError::Parser(format!(
                    "unexpected EOF: expected one of: {expected_tokens_str}"
                )))
            } else {
                Ok(TokenKind::STOP)
            }
        }
    }
}

pub fn log_error(
    token_pos: Range<usize>,
    err: CompilerError,
    offset: usize,
    source: &str,
    trace: bool,
) {
    let path = SOURCE_CODE_PATH.with(|path| path.borrow().clone().unwrap());

    let (line_starts, (line_in_file, col_in_file)) =
        pos_to_line_col(source, token_pos.start + offset);

    let line_start = line_starts[line_in_file - 1];
    let line_end = source[line_start..]
        .find('\n')
        .map(|e| line_start + e)
        .unwrap_or(source.len());
    let line_text = &source[line_start..line_end];

    let span_len = std::cmp::min(
        token_pos.end - token_pos.start,
        line_text.len() - (col_in_file - 1),
    );
    let mut underline = String::new();
    underline.push_str(&" ".repeat(col_in_file - 1));
    underline.push_str(&"^".repeat(span_len));
    eprintln!("{}: {}", "error".red().bold(), err.to_string().bold());
    eprintln!(
        "  --> {}:{}:{}",
        path.to_str().unwrap_or("").bright_blue(),
        line_in_file.blue(),
        col_in_file.blue()
    );
    if trace {
        eprintln!("   {}", "|".dimmed());
        eprintln!(
            "{:>3}{} {}",
            line_in_file.to_string().blue(),
            "|".dimmed(),
            line_text
        );
        eprintln!(
            "   {} {} {}",
            "|".dimmed(),
            underline.bold().red(),
            err.bold().red()
        );
    } else {
        eprintln!("   {}", err.bold().red());
    }
    eprintln!()
}

fn pos_to_line_col(source: &str, pos: usize) -> (Vec<usize>, (usize, usize)) {
    let mut line_starts = vec![0];
    for (i, ch) in source.char_indices() {
        if ch == '\n' {
            line_starts.push(i + 1);
        }
    }

    let line = line_starts.partition_point(|&start| start <= pos);

    let col = pos - line_starts[line - 1] + 1;

    (line_starts, (line, col))
}

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
            Self::TokenSemicolon => "\";\"",
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
