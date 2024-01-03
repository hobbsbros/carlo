//! Parser for the Carlo language.

mod expression;
mod prefix_parselet;
mod infix_parselet;

// Prefix parselets
mod number_parselet;
mod identifier_parselet;
mod assignment_parselet;

// Infix parselets
mod reassignment_parselet;

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

use reassignment_parselet::ReassignmentParselet;

pub struct Parser {
    prefix_parselets: HashMap<TokenClass, Box<dyn PrefixParselet>>,
    infix_parselets: HashMap<TokenClass, Box<dyn InfixParselet>>,
    pub debug: bool,
}

impl Parser {
    /// Constructs a new parser.
    pub fn new(debug: bool) -> Self {
        use TokenClass::*;

        let mut prefix_parselets: HashMap<TokenClass, Box<dyn PrefixParselet>> = HashMap::new();
        let mut infix_parselets: HashMap<TokenClass, Box<dyn InfixParselet>> = HashMap::new();

        // Declarative grammar: prefix parselets
        prefix_parselets.insert(Number, Box::new(NumberParselet {}));
        prefix_parselets.insert(Identifier, Box::new(IdentifierParselet {}));
        prefix_parselets.insert(Let, Box::new(AssignmentParselet {}));

        // Declarative grammar: infix parselet
        infix_parselets.insert(Assignment, Box::new(ReassignmentParselet {}));

        Self {
            prefix_parselets,
            infix_parselets,
            debug,
        }
    }

    /// Parses a tokenstream.
    pub fn parse(&self, input: String) -> Vec<Expression> {
        let mut expressions = Vec::new();
        let mut tokenstream = Tokenstream::from(&input, self.debug);

        while let Some(token) = tokenstream.peek() {
            let expr = self.parse_expr(&mut tokenstream, 0);
            expressions.push(expr);
        }

        expressions
    }

    /// Parses an expression.
    fn parse_expr(&self, tokenstream: &mut Tokenstream, _precedence: u8) -> Expression {
        let token = tokenstream.next_unwrap();

        let prefix_parselet = match self.prefix_parselets.get(&token.class) {
            Some (p) => p,
            None => Error::CouldNotParse (&token.value).throw(),
        };

        let expression = prefix_parselet.parse(
            tokenstream,
            self,
            token,
        );

        let token = match tokenstream.peek() {
            Some (t) => t,
            None => return expression,
        };

        let infix_parselet = match self.infix_parselets.get(&token.class) {
            Some (p) => p,
            None => return expression,
        };

        tokenstream.next();

        infix_parselet.parse(
            tokenstream,
            self,
            expression,
            token,
        )
    }
}