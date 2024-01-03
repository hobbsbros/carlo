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
mod binary_operation_parselet;

use std::collections::HashMap;

pub use crate::{
    Error,
    Token,
    TokenClass,
    Tokenstream,
};

pub use expression::{
    BinaryOperation,
    Expression,
};

use prefix_parselet::PrefixParselet;
use infix_parselet::InfixParselet;

use number_parselet::NumberParselet;
use identifier_parselet::IdentifierParselet;
use assignment_parselet::AssignmentParselet;

use reassignment_parselet::ReassignmentParselet;
use binary_operation_parselet::BinaryOperationParselet;

/// Abstracts over the Carlo parser.
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
        infix_parselets.insert(Plus, Box::new(BinaryOperationParselet {}));
        infix_parselets.insert(Minus, Box::new(BinaryOperationParselet {}));
        infix_parselets.insert(Times, Box::new(BinaryOperationParselet {}));
        infix_parselets.insert(Divide, Box::new(BinaryOperationParselet {}));

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

        while let Some(_) = tokenstream.peek() {
            let expr = self.parse_expr(&mut tokenstream, 0, 0);
            expressions.push(expr);
        }

        expressions
    }

    /// Parses an expression.
    fn parse_expr(&self, tokenstream: &mut Tokenstream, precedence: u8, nesting: usize) -> Expression {
        let token = tokenstream.next_unwrap();

        let prefix_parselet = match self.prefix_parselets.get(&token.class) {
            Some (p) => p,
            None => Error::CouldNotParse (&token.value).throw(),
        };

        let mut expression = prefix_parselet.parse(
            tokenstream,
            self,
            token,
            nesting,
        );

        // Indent debug statements
        let mut indent = String::new();
        for _ in 0..nesting {
            indent.push_str("    ");
        }

        if self.debug {
            println!("{}Parsing precedence: {}", indent, precedence);
            println!("{}Tokenstream precedence: {}", indent, tokenstream.precedence());
            println!();
        }

        while precedence < tokenstream.precedence() {
            let token = match tokenstream.peek() {
                Some (t) => t,
                None => return expression,
            };

            if self.debug {
                println!("{}Conducting infix parsing with token {}", indent, token);
                println!();
            }

            let infix_parselet = match self.infix_parselets.get(&token.class) {
                Some (p) => p,
                None => return expression,
            };
    
            tokenstream.next();
    
            expression = infix_parselet.parse(
                tokenstream,
                self,
                expression,
                token,
                nesting,
            );
        }

        if self.debug {
            println!("{}Parsing complete", indent);
            println!();
        }

        expression
    }
}