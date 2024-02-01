//! Defines a paragraph parselet.

use super::{
    Expression,
    Parser,
    PrefixParselet,
    Token,
    Tokenstream,
};

pub struct ParagraphParselet {}

impl PrefixParselet for ParagraphParselet {
    fn parse(&self, _tokenstream: &mut Tokenstream, _parser: &Parser, token: Token, _nesting: usize) -> Expression {
        use Expression::*;

        Paragraph (token.value.trim().to_string())
    }
}