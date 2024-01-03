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
    fn parse(&self, tokenstream: &mut Tokenstream, parser: &Parser, token: Token) -> Expression {
        use Expression::*;

        if parser.debug {
            println!();
            println!("Parsing number near token {}", token);
        }

        match str::parse::<i64>(&token.value) {
            Ok (i) => Integer (i),
            _ => match str::parse::<f64>(&token.value) {
                Ok (f) => Float (f),
                _ => Error::CouldNotParseNumber (&token.value).throw(),
            }
        }
    }
}