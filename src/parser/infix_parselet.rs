//! Abstracts over infix parselets.

use super::{
    Error,
    Expression,
    Parser,
    Token,
    Tokenstream,
};

pub trait InfixParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, parser: &Parser, left: Expression, token: Token) -> Expression;
}