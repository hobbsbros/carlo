//! Parser for the Carlo language.

mod expression;
mod prefix_parselet;
mod infix_parselet;

// Prefix parselets
mod number_parselet;
mod identifier_parselet;

// Infix parselets
mod assignment_parselet;

use std::collections::HashMap;

pub use crate::{
    Error,
    Token,
    TokenClass,
    Tokenstream,
};

pub use expression::Expression;

use prefix_parselet::PrefixParselet;
use infix_parselet::InfixParselet;

use number_parselet::NumberParselet;
use identifier_parselet::IdentifierParselet;

use assignment_parselet::AssignmentParselet;

pub struct Parser {
    prefix_parselets: HashMap<TokenClass, Box<dyn PrefixParselet>>,
    infix_parselets: HashMap<TokenClass, Box<dyn InfixParselet>>,
}

impl Parser {
    /// Constructs a new parser.
    pub fn new() -> Self {
        use TokenClass::*;

        let mut prefix_parselets: HashMap<TokenClass, Box<dyn PrefixParselet>> = HashMap::new();
        let mut infix_parselets: HashMap<TokenClass, Box<dyn InfixParselet>> = HashMap::new();

        // Declarative grammar: prefix parselets
        prefix_parselets.insert(Number, Box::new(NumberParselet {}));
        prefix_parselets.insert(Identifier, Box::new(IdentifierParselet {}));

        // Declarative grammar: infix parselet
        infix_parselets.insert(Assignment, Box::new(AssignmentParselet {}));

        Self {
            prefix_parselets,
            infix_parselets,
        }
    }

    /// Parses a tokenstream.
    pub fn parse(&self, input: String) -> Vec<Expression> {
        let mut expressions = Vec::new();
        let mut tokenstream = Tokenstream::from(&input);

        while let Some(token) = tokenstream.peek() {
            tokenstream.next();

            let expr = self.parse_expr(&mut tokenstream, token);
            expressions.push(expr);
        }

        expressions
    }

    /// Parses an expression.
    fn parse_expr(&self, tokenstream: &mut Tokenstream, token: Token) -> Expression {
        let prefix_parselet = match self.prefix_parselets.get(&token.class) {
            Some (p) => p,
            None => Error::CouldNotParse (&token.value).throw(),
        };

        let mut expression = prefix_parselet.parse(
            tokenstream,
            self,
            token,
        );

        while let Some(token) = tokenstream.peek() {
            tokenstream.next();

            let infix_parselet = match self.infix_parselets.get(&token.class) {
                Some (p) => p,
                None => Error::CouldNotParse (&token.value).throw(),
            };

            expression = infix_parselet.parse(
                tokenstream,
                self,
                expression,
                token,
            );
        }

        expression
    }
}