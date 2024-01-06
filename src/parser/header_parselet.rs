//! Defines a header parselet.

use super::{
    Expression,
    Parser,
    PrefixParselet,
    Token,
    Tokenstream,
};

pub struct HeaderParselet {}

impl PrefixParselet for HeaderParselet {
    fn parse(&self, _tokenstream: &mut Tokenstream, _parser: &Parser, token: Token, _nesting: usize) -> Expression {
        use Expression::*;

        Header (token.value.trim().to_owned())
    }
}