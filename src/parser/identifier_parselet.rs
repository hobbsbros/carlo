//! Defines an identifier parselet.

use super::{
    Expression,
    Parser,
    PrefixParselet,
    Token,
    Tokenstream,
};

pub struct IdentifierParselet {}

impl PrefixParselet for IdentifierParselet {
    fn parse(&self, _tokenstream: &mut Tokenstream, _parser: &Parser, token: Token, _nesting: usize) -> Expression {
        use Expression::*;

        Identifier (token.value.to_owned())
    }
}