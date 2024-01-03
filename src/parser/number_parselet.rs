//! Defines a numerical value parselet.

use super::{
    Error,
    Expression,
    Parser,
    PrefixParselet,
    Token,
    Tokenstream,
};

pub struct NumberParselet {}

impl PrefixParselet for NumberParselet {
    fn parse(&self, _tokenstream: &mut Tokenstream, _parser: &Parser, token: Token, _nesting: usize) -> Expression {
        use Expression::*;

        match str::parse::<i64>(&token.value) {
            Ok (i) => Integer (i),
            _ => match str::parse::<f64>(&token.value) {
                Ok (f) => Float (f),
                _ => Error::CouldNotParseNumber (&token.value).throw(),
            }
        }
    }
}