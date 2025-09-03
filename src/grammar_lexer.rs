use crate::grammar::{State, TokenKind};

use rustemo::{Context, LRContext, Lexer, Location, Position, Token};
use std::iter;

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
        _expected_tokens: Vec<(TokenKind, bool)>,
    ) -> Box<dyn Iterator<Item = Token<'i, Self::Input, TokenKind>> + 'i> {
        let mut pos = context.position();

        let value;
        let token;

        if context.position() >= input.len() || input.is_empty() {
            value = "";
            token = TokenKind::STOP
        } else {
            let input = input.get(context.position()..input.len()).unwrap();
            let mut lexer = crate::lex::Lexer::new(input);
            token = lexer.yylex().unwrap_or(TokenKind::STOP);

            let range = lexer.yytextpos();
            pos += lexer.yytextpos().start;
            value = unsafe { input.get_unchecked(range) };
        }

        context.set_position(pos);

        Box::new(iter::once(Token {
            kind: token,
            value,
            location: Location {
                start: Position::Position(context.position()),
                end: None,
            },
        }))
    }
}
