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
    fn parse(&self, tokenstream: &mut Tokenstream, parser: &Parser, token: Token, nesting: usize) -> Expression {
        use Expression::*;

        // Parse left
        let left = &tokenstream.get(TokenClass::Identifier).value;

        // Discard equals sign
        tokenstream.get(TokenClass::Assignment);

        // Parse right
        let right = parser.parse_expr(tokenstream, token.precedence(), nesting + 1);

        Assignment {
            left: left.to_owned(),
            right: Box::new(right),
        }
    }
}