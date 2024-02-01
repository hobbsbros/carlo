//! Defines a header parselet.

use super::{
    Expression,
    Parser,
    PrefixParselet,
    Token,
    Tokenstream,
};

pub struct HeaderParselet {}

impl PrefixParselet for HeaderParselet {
    fn parse(&self, _tokenstream: &mut Tokenstream, _parser: &Parser, token: Token, _nesting: usize) -> Expression {
        use Expression::*;

        let mut chars = token.value.chars();

        if token.value.starts_with("@@@") {
            chars.next();
            chars.next();
            chars.next();

            Subsubheader (chars.collect::<String>().trim().to_owned())
        } else if token.value.starts_with("@@") {
            chars.next();
            chars.next();

            Subheader (chars.collect::<String>().trim().to_owned())
        } else if token.value.starts_with("@") {
            chars.next();

            Header (chars.collect::<String>().trim().to_owned())
        } else {
            unreachable!()
        }
    }
}