//! Defines a reassignment parselet.

use super::{
    Error,
    Expression,
    Parser,
    InfixParselet,
    Token,
    TokenClass,
    Tokenstream,
};

pub struct ReassignmentParselet {}

impl InfixParselet for ReassignmentParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, parser: &Parser, left: Expression, token: Token) -> Expression {
        use Expression::*;

        if parser.debug {
            println!();
            println!("Parsing reassignment near token {}", token);
        }

        let id = &tokenstream.get(TokenClass::Identifier).value;

        Reassignment {
            left: id.to_owned(),
            right: Box::new(parser.parse_expr(tokenstream, 0)),
        }
    }
}