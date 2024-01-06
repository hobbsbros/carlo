//! Defines a symbolic parselet.

use super::{
    Expression,
    Parser,
    PrefixParselet,
    Token,
    TokenClass,
    Tokenstream,
};

pub struct SymbolicParselet {}

impl PrefixParselet for SymbolicParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, _parser: &Parser, _token: Token, _nesting: usize) -> Expression {
        use Expression::*;

        let variable = &tokenstream.get(TokenClass::Identifier).value;

        Symbolic (variable.to_owned())
    }
}