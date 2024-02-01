//! Defines an assignment parselet.

use super::{
    Error,
    Expression,
    Parser,
    PrefixParselet,
    Token,
    TokenClass,
    Tokenstream,
};

pub struct ParenthesisParselet {}

impl PrefixParselet for ParenthesisParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, parser: &Parser, token: Token, nesting: usize) -> Expression {
        use TokenClass::CloseParen;
        
        let inside = parser.parse_expr(tokenstream, token.precedence(), nesting + 1);

        let next = tokenstream.next_unwrap().class;
        if next != CloseParen {
            Error::Expected (CloseParen, next).warn();
            return Expression::Null;
        }

        inside
    }
}