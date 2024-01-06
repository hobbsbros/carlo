//! Defines Carlo language expressions.

use std::fmt;

use crate::{
    BinaryOperation,
    UNITS,
};

#[derive(Clone, Debug)]
/// Enumerates the expression available to the Carlo parser.
pub enum Expression {
    /// Assignment
    Assignment {
        left: String,
        right: Box<Expression>,
    },

    /// Reassignment
    Reassignment {
        left: String,
        right: Box<Expression>,
    },

    /// 64-bit dimensioned floating-point
    Float {
        value: f64,
        kg: f64,
        m: f64,
        s: f64,
        a: f64,
        k: f64,
        mol: f64,
    },

    /// Identifier
    Identifier (String),

    /// Symbolic identifier
    Symbolic (String),

    /// Binary operation
    BinOp {
        left: Box<Expression>,
        oper: BinaryOperation,
        right: Box<Expression>,
    },

    /// Null
    Null,
}

impl Expression {
    pub fn latex(&self) -> String {
        use Expression::*;

        let string = match self {
            Assignment {
                left,
                right,
            } => {
                format!("{} = {}", left, right.latex())
            },
            Reassignment {
                left,
                right,
            } => {
                format!("{} = {}", left, right.latex())
            },
            Float {
                value,
                kg,
                m,
                s,
                a,
                k,
                mol,
            } => {
                latex_unit(*value, *kg, *m, *s, *a, *k, *mol)
            },
            Identifier (s) => {
                format!("{}", s)
            },
            Symbolic (s) => {
                format!("{}", s)
            },
            BinOp {
                left,
                oper,
                right,
            } => {
                format!("{}{}{}", left.latex(), oper.latex(), right.latex())
            },
            Null => String::new(),
        };

        string
    }
}

fn format_unit(
    mut value: f64,
    mut kg: f64,
    mut m: f64,
    mut s: f64,
    mut a: f64,
    mut k: f64,
    mut mol: f64,
) -> String {
    let mut output = String::new();

    // Iterate through units and add values
    for (name, unit) in UNITS {
        if (kg, m, s, a, k, mol) == (unit.1, unit.2, unit.3, unit.4, unit.5, unit.6) {
            value /= unit.0;

            output.push_str(&format!(" {}", name));

            kg -= unit.1;
            m -= unit.2;
            s -= unit.3;
            a -= unit.4;
            k -= unit.5;
            mol -= unit.6;
        }
    }

    for (unit, pow) in [
        ("kg", kg),
        ("m", m),
        ("s", s),
        ("A", a),
        ("K", k),
        ("mol", mol),
    ] {
        if pow == 1.0 {
            output.push_str(&format!(" {}", unit));
        } else if pow != 0.0 {
            output.push_str(&format!(" {}^{}", unit, pow));
        }
    }

    format!("{}{}", value, output)   
}

fn latex_unit(
    mut value: f64,
    mut kg: f64,
    mut m: f64,
    mut s: f64,
    mut a: f64,
    mut k: f64,
    mut mol: f64,
) -> String {
    let mut output = String::new();

    // Iterate through units and add values
    for (name, unit) in UNITS {
        if (kg, m, s, a, k, mol) == (unit.1, unit.2, unit.3, unit.4, unit.5, unit.6) {
            value /= unit.0;

            output.push_str(&format!(" \\; \\mathrm{{{}}}", name));

            kg -= unit.1;
            m -= unit.2;
            s -= unit.3;
            a -= unit.4;
            k -= unit.5;
            mol -= unit.6;
        }
    }

    for (unit, pow) in [
        ("kg", kg),
        ("m", m),
        ("s", s),
        ("A", a),
        ("K", k),
        ("mol", mol),
    ] {
        if pow == 1.0 {
            output.push_str(&format!(" \\; \\mathrm{{{}}}", unit));
        } else if pow != 0.0 {
            output.push_str(&format!(" \\; \\mathrm{{{}^{{{}}}}}", unit, pow));
        }
    }

    format!("{}{}", value, output)
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expression::*;

        let string = match self {
            Assignment {
                left,
                right,
            } => {
                format!("{} = {}", left, right)
            },
            Reassignment {
                left,
                right,
            } => {
                format!("{} = {}", left, right)
            },
            Float {
                value,
                kg,
                m,
                s,
                a,
                k,
                mol,
            } => {
                format_unit(*value, *kg, *m, *s, *a, *k, *mol)
            },
            Identifier (s) => {
                format!("{}", s)
            },
            Symbolic (s) => {
                format!("{}", s)
            },
            BinOp {
                left,
                oper,
                right,
            } => {
                format!("{} {} {}", left, oper, right)
            },
            Null => "Null".to_string(),
        };

        write!(f, "{}", string)
    }
}