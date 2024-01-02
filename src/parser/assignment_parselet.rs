//! Defines a numerical value parselet.

use super::{
    Error,
    Expression,
    Parser,
    InfixParselet,
    Token,
    TokenClass,
    Tokenstream,
};

pub struct AssignmentParselet {}

impl InfixParselet for AssignmentParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, parser: &Parser, left: Expression, token: Token) -> Expression {
        use Expression::*;

        match left {
            Identifier (id) => Assignment {
                left: id.to_owned(),
                right: Box::new(parser.parse_expr(tokenstream, token)),
            },
            _ => todo!(),
        }
    }
}