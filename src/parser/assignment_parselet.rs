//! Defines an assignment parselet.

use super::{
    Error,
    Expression,
    Parser,
    PrefixParselet,
    Token,
    TokenClass,
    Tokenstream,
};

pub struct AssignmentParselet {}

impl PrefixParselet for AssignmentParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, parser: &Parser, token: Token) -> Expression {
        use Expression::*;

        if parser.debug {
            println!();
            println!("Parsing assignment near token {}", token);
        }

        // Parse left
        let left = &tokenstream.get(TokenClass::Identifier).value;

        // Discard equals sign
        tokenstream.get(TokenClass::Assignment);

        // Parse right
        let right = parser.parse_expr(tokenstream, 0);

        Assignment {
            left: left.to_owned(),
            right: Box::new(right),
        }
    }
}