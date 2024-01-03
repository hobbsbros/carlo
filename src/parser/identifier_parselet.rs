//! Defines an identifier parselet.

use super::{
    Error,
    Expression,
    Parser,
    PrefixParselet,
    Token,
    Tokenstream,
};

pub struct IdentifierParselet {}

impl PrefixParselet for IdentifierParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, parser: &Parser, token: Token, _nesting: usize) -> Expression {
        use Expression::*;

        Expression::Identifier (token.value.to_owned())
    }
}