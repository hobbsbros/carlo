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
    fn lookup(&self, name: &String) -> Option<Expression> {
        self.variables.get(name).cloned()
    }

    /// Simplifies an expression in this environment.
    fn simplify(&mut self, expr: &Expression, toplevel: bool, resolve_names: bool) -> Expression {
        use Expression::*;

        match expr {
            Assignment {
                left,
                right,
            } => {
                // Simplify the RHS
                let sr = self.simplify(right, false, false);
                
                self.register(&left, &sr);
                sr.to_owned()
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
                let sr = self.simplify(right, false, false);
                
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
            Identifier (s) => if resolve_names {
                match self.lookup(&s) {
                    Some (e) => self.simplify(&e, false, true),
                    None => if toplevel {
                        Error::UndeclaredVariable (&s).warn();
                        Null
                    } else {
                        expr.to_owned()
                    },
                }
            } else {
                expr.to_owned()
            },
            Symbolic (s) => match self.lookup(&s) {
                Some (e) => self.simplify(&e, false, false),
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
                let sl = self.simplify(left, false, resolve_names);
                let sr = self.simplify(right, false, resolve_names);
                oper.simplify(&sl, &sr)
            },
            Null => Null,
        }
    }

    /// Simplifies an expression in this environment for a LaTeX document.
    fn simplify_latex(&mut self, expr: &Expression, toplevel: bool, resolve_names: bool) -> Expression {
        use Expression::*;

        match expr {
            Assignment {
                left,
                right,
            } => {
                // Simplify the RHS
                let sr = self.simplify(right, false, false);
                
                self.register(&left, &sr);
                
                Assignment {
                    left: left.to_owned(),
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
                let sr = self.simplify(right, false, false);
                
                self.register(&left, &sr);
                
                Reassignment {
                    left: left.to_owned(),
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
            Identifier (s) => if resolve_names {
                match self.lookup(&s) {
                    Some (e) => self.simplify(&e, false, true),
                    None => if toplevel {
                        Error::UndeclaredVariable (&s).warn();
                        Null
                    } else {
                        expr.to_owned()
                    },
                }
            } else {
                expr.to_owned()
            },
            Symbolic (s) => match self.lookup(&s) {
                Some (e) => self.simplify(&e, false, false),
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
                let sl = self.simplify(left, false, resolve_names);
                let sr = self.simplify(right, false, resolve_names);
                oper.simplify(&sl, &sr)
            },
            Null => Null,
        }
    }

    /// Evaluates a series of statements in this environment.
    pub fn evaluate(&mut self, expressions: &Vec<Expression>) -> String {
        let mut output = String::new();

        for expr in expressions {
            let out = self.simplify(expr, true, true);

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
            let out = self.simplify_latex(expr, true, true);

            if let Expression::Null = expr {
                // Do not print Null
            } else {
                output.push_str(&format!("$$\n{}\n$$\n", out.latex()));
            }
        }

        output
    }
}