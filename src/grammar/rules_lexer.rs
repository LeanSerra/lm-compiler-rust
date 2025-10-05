use super::rules::{State, TokenKind};
use crate::compiler::error::{CompilerError, log_error_and_exit};
use rustemo::{Context, LRContext, Lexer, Location, Position, Token};
use std::iter;

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
            let mut lexer = crate::lexer::lex::Lexer::new(trimmed_input, pos);
            token =
                match validate_and_get_next_token(&mut lexer, expected_tokens, expected_tokens_str)
                {
                    Ok(tok) => tok,
                    Err(e) => log_error_and_exit(lexer.yytextpos(), e, pos, true),
                };
            let range = lexer.yytextpos();
            pos += range.start;
            value = trimmed_input.get(range).unwrap();
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
    lexer: &mut crate::lexer::lex::Lexer,
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
            if let crate::lexer::lex::Error::Unmatch = e {
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
