//! Abstracts over prefix parselets.

use super::{
    Error,
    Expression,
    Parser,
    Token,
    Tokenstream,
};

pub trait PrefixParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, parser: &Parser, token: Token, nesting: usize) -> Expression;
}