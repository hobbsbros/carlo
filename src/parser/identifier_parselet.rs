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
    fn parse(&self, tokenstream: &mut Tokenstream, parser: &Parser, token: Token) -> Expression {
        use Expression::*;

        if parser.debug {
            println!();
            println!("Parsing identifier near token {}", token);
        }

        Expression::Identifier (token.value.to_owned())
    }
}