//! Defines a numerical value parselet.

use super::{
    Error,
    Expression,
    Parser,
    PrefixParselet,
    Token,
    TokenClass,
    Tokenstream,
};

pub struct NumberParselet {}

impl PrefixParselet for NumberParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, _parser: &Parser, token: Token, _nesting: usize) -> Expression {
        use Expression::*;

        let mut kg = 0.0;
        let mut m = 0.0;
        let mut s = 0.0;

        while let Some(t) = tokenstream.peek() {
            println!("{}", t);

            if t.check(TokenClass::Kilogram) {
                tokenstream.next();

                kg = if t.value.len() == 2 {
                    1.0
                } else {
                    match str::parse::<f64>(&t.value[2..]) {
                        Ok (f) => f,
                        Err (_) => Error::CouldNotParseExponent (&t.value).throw(),
                    }
                }
            } else if t.check(TokenClass::Meter) {
                tokenstream.next();

                m = if t.value.len() == 1 {
                    1.0
                } else {
                    match str::parse::<f64>(&t.value[1..]) {
                        Ok (f) => f,
                        Err (_) => Error::CouldNotParseExponent (&t.value).throw(),
                    }
                }
            } else if t.check(TokenClass::Second) {
                tokenstream.next();

                s = if t.value.len() == 1 {
                    1.0
                } else {
                    match str::parse::<f64>(&t.value[1..]) {
                        Ok (f) => f,
                        Err (_) => Error::CouldNotParseExponent (&t.value).throw(),
                    }
                }
            } else {
                break;
            }
        }

        match str::parse::<f64>(&token.value) {
            Ok (f) => Float {
                value: f,
                kg,
                m,
                s,
            },
            _ => Error::CouldNotParseNumber (&token.value).throw(),
        }
    }
}