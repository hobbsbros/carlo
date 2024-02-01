//! Defines a function call parselet.

use super::{
    Expression,
    Parser,
    InfixParselet,
    Token,
    TokenClass,
    Tokenstream,
};

pub struct FunctionCallParselet {}

impl InfixParselet for FunctionCallParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, parser: &Parser, left: Expression, token: Token, nesting: usize) -> Expression {
        use Expression::*;

        // Parse left
        let name = match left {
            Identifier (i) => i,
            _ => todo!(),
        };

        // Set up list of arguments
        let mut arguments = Vec::new();

        // Parse right
        while let Some (t) = tokenstream.peek() {
            // Break on closing parenthesis
            if t.class == TokenClass::CloseParen {
                tokenstream.next();
                break;
            } else if t.class == TokenClass::Comma {
                tokenstream.next();
            }

            let argument = parser.parse_expr(tokenstream, token.precedence() - 1, nesting + 1);

            arguments.push(argument);
        }

        FnCall {
            name,
            arguments,
        }
    }
}