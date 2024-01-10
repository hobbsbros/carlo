//! Defines a fully resolved symbolic parselet.

use super::{
    Expression,
    Parser,
    PrefixParselet,
    Token,
    TokenClass,
    Tokenstream,
};

pub struct FullSymbolicParselet {}

impl PrefixParselet for FullSymbolicParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, _parser: &Parser, _token: Token, _nesting: usize) -> Expression {
        use Expression::*;

        let variable = &tokenstream.get(TokenClass::Identifier).value;

        FullSymbolic (variable.to_owned())
    }
}