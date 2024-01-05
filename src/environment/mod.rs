//! Environment manager for the Carlo language.

use std::collections::HashMap;

use crate::{
    Error,
    Expression,
};

pub struct Environment {
    variables: HashMap<String, Expression>,
}

impl Environment {
    /// Constructs a new environment.
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Registers a variable in this environment.
    fn register(&mut self, name: &String, value: &Expression) {
        self.variables.insert(name.to_owned(), value.to_owned());
    }

    /// Looks up a variable in this environment.
    fn lookup(&self, name: &String) -> Expression {
        match self.variables.get(name) {
            Some (expr) => expr.to_owned(),
            None => {
                Error::UndeclaredVariable (name).warn();
                Expression::Null
            },
        }
    }

    /// Simplifies an expression in this environment.
    fn simplify(&mut self, expr: &Expression) -> Expression {
        use Expression::*;

        match expr {
            Assignment {
                left,
                right,
            } => {
                let sr = self.simplify(right);
                self.register(&left, &sr);
                sr.to_owned()
            },
            Reassignment {
                left,
                right,
            } => {
                let _ = self.lookup(&left);
                let sr = self.simplify(right);
                self.register(&left, &sr);
                sr.to_owned()
            },
            Float {
                value: _,
                kg: _,
                m: _,
                s: _,
                a: _,
                k: _,
                mol: _,
            } => expr.to_owned(),
            Identifier (s) => self.lookup(&s),
            BinOp {
                left,
                oper,
                right,
            } => {
                let sl = self.simplify(left);
                let sr = self.simplify(right);
                oper.simplify(&sl, &sr)
            },
            Null => Null,
        }
    }

    /// Evaluates a series of statements in this environment.
    pub fn evaluate(&mut self, expressions: &Vec<Expression>) -> String {
        let mut output = String::new();

        for expr in expressions {
            let out = self.simplify(expr);
            output.push_str(&format!("{}\n", out));
        }

        output
    }
}