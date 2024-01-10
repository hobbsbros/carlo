//! Environment manager for the Carlo language.

use std::collections::HashMap;

use crate::{
    Error,
    Expression,
};

#[derive(Clone, Copy)]
/// Denote the type of symbolic resolution.
pub enum Resolution {
    NoResolve,
    SymbolsOnly,
    Numeric,
}

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
    fn lookup(&self, name: &String) -> Option<Expression> {
        self.variables.get(name).cloned()
    }

    /// Simplifies an expression in this environment.
    fn simplify(&mut self, expr: &Expression, resolve_names: Resolution) -> Expression {
        use Expression::*;
        use Resolution::*;

        match expr {
            Assignment {
                left,
                right,
            } => {
                // Simplify the RHS
                let sr = self.simplify(right, NoResolve);
                
                self.register(&left, &sr);
                
                Assignment {
                    left: left.to_string(),
                    right: Box::new(sr.to_owned()),
                }
            },
            Reassignment {
                left,
                right,
            } => {
                // Make sure this variable exists
                match self.lookup(&left) {
                    Some (_) => (),
                    None => {
                        Error::UndeclaredVariable (&left).warn();
                        return Null;
                    },
                };

                // Simplify the RHS
                let sr = self.simplify(right, NoResolve);
                
                self.register(&left, &sr);
                
                Reassignment {
                    left: left.to_string(),
                    right: Box::new(sr.to_owned()),
                }
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
            Identifier (s) => match resolve_names {
                NoResolve => expr.to_owned(),
                SymbolsOnly => match self.lookup(&s) {
                    Some (e) => match e.is_numeric() {
                        true => expr.to_owned(),
                        false => self.simplify(&e, SymbolsOnly),
                    },
                    None => expr.to_owned(),
                },
                Numeric => match self.lookup(&s) {
                    Some (e) => self.simplify(&e, Numeric),
                    None => {
                        Error::UndeclaredVariable (&s).warn();
                        Null
                    },
                },
            }
            Symbolic (s) => match self.lookup(&s) {
                Some (e) => Reassignment {
                    left: s.to_string(),
                    right: Box::new(self.simplify(&e, NoResolve)),
                },
                None => {
                    Error::UndeclaredVariable (&s).warn();
                    Null
                },
            },
            FullSymbolic (s) => match self.lookup(&s) {
                Some (e) => Reassignment {
                    left: s.to_string(),
                    right: Box::new(self.simplify(&e, SymbolsOnly)),
                },
                None => {
                    Error::UndeclaredVariable (&s).warn();
                    Null
                },
            },
            BinOp {
                left,
                oper,
                right,
            } => {
                let sl = self.simplify(left, resolve_names);
                let sr = self.simplify(right, resolve_names);
                oper.simplify(&sl, &sr)
            },
            Header (_) => expr.to_owned(),
            Null => Null,
        }
    }

    /// Evaluates a series of statements in this environment.
    pub fn evaluate(&mut self, expressions: &Vec<Expression>) -> String {
        let mut output = String::new();

        for expr in expressions {
            let out = self.simplify(expr, Resolution::Numeric);

            if let Expression::Null = expr {
                // Do not print Null
            } else {
                output.push_str(&format!("{}\n", out));
            }
        }

        output
    }

    /// Evaluates a series of statements in this environment and returns LaTeX.
    pub fn latex_evaluate(&mut self, expressions: &Vec<Expression>) -> String {
        let mut output = String::new();

        for expr in expressions {
            let out = self.simplify(expr, Resolution::Numeric);

            if let Expression::Null = expr {
                // Do not print Null
            } else {
                let latex = out.latex(true);
                if !latex.starts_with("\n\\section") {
                    output.push_str(&format!("$$\n{}\n$$\n", out.latex(true)));
                } else {
                    output.push_str(&format!("\n{}\n\n", out.latex(true)));
                }
            }
        }

        output
    }
}