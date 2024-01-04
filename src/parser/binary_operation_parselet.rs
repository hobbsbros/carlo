//! Defines a binary operation parselet.

use super::{
    BinaryOperation,
    Expression,
    Parser,
    InfixParselet,
    Token,
    TokenClass,
    Tokenstream,
};

pub struct BinaryOperationParselet {}

impl InfixParselet for BinaryOperationParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, parser: &Parser, left: Expression, token: Token, nesting: usize) -> Expression {
        use Expression::*;
        use TokenClass::*;
        use BinaryOperation::*;

        let oper = match token.class {
            Plus => Add,
            Minus => Sub,
            Times => Mul,
            Divide => Div,
            _ => unreachable!(),
        };

        BinOp {
            left: Box::new(left),
            oper,
            right: Box::new(parser.parse_expr(tokenstream, token.precedence(), nesting + 1)),
        }.simplify()
    }
}