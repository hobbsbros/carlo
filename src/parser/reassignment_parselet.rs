//! Defines a reassignment parselet.

use super::{
    Expression,
    Parser,
    InfixParselet,
    Token,
    Tokenstream,
};

pub struct ReassignmentParselet {}

impl InfixParselet for ReassignmentParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, parser: &Parser, left: Expression, token: Token, nesting: usize) -> Expression {
        use Expression::*;

        let right = parser.parse_expr(tokenstream, token.precedence(), nesting + 1);

        let left = match left {
            Identifier (i) => i,
            _ => unreachable!(),
        };

        Reassignment {
            left: left.to_owned(),
            right: Box::new(right),
        }
    }
}