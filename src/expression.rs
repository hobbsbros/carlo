//! Defines Carlo language expressions.

use std::fmt;

use crate::{
    Error,
    UNITS,
};

#[derive(Clone, Copy, Debug)]
/// Defines binary operations in Carlo.
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOperation {
    /// Simplifies this binary operation.
    pub fn simplify(&self, left: &Expression, right: &Expression) -> Expression {
        use Expression::*;

        if let Float {
            value: l_value,
            kg: l_kg,
            m: l_m,
            s: l_s,
            a: l_a,
            k: l_k,
            mol: l_mol,
        } = left {
            if let Float {
                value: r_value,
                kg: r_kg,
                m: r_m,
                s: r_s,
                a: r_a,
                k: r_k,
                mol: r_mol,
            } = right {
                // Left & Right are numeric
                Float {
                    value: self.oper_value(l_value, r_value),
                    kg: self.oper_unit("kg", l_kg, r_kg),
                    m: self.oper_unit("m", l_m, r_m),
                    s: self.oper_unit("s", l_s, r_s),
                    a: self.oper_unit("A", l_a, r_a),
                    k: self.oper_unit("K", l_k, r_k),
                    mol: self.oper_unit("mol", l_mol, r_mol),
                }
            } else {
                // Left is numeric, Right is not
                BinOp {
                    left: Box::new(left.to_owned()),
                    oper: *self,
                    right: Box::new(right.to_owned()),
                }
            }
        } else {
            BinOp {
                left: Box::new(left.to_owned()),
                oper: *self,
                right: Box::new(right.to_owned()),
            }
        }
    }

    /// Operate on two numbers.
    pub fn oper_value(&self, left: &f64, right: &f64) -> f64 {
        use BinaryOperation::*;

        match self {
            Add => left + right,
            Sub => left - right,
            Mul => left * right,
            Div => left / right,
        }
    }

    /// Operate on two units.
    pub fn oper_unit(&self, unit: &str, left: &f64, right: &f64) -> f64 {
        use BinaryOperation::*;

        match self {
            Add | Sub => if left == right {
                return *left
            } else {
                Error::UnmatchedUnits (unit, &left.to_string(), &right.to_string()).warn();
                return 0.0;
            },
            Mul => left + right,
            Div => left - right,
        }
    }
}

impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BinaryOperation::*;

        let s = match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
        };

        write!(f, "{}", s)
    }
}

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
                latex_unit(*value, *kg, *m, *s, *a, *k, *mol)
            },
            Identifier (s) => {
                format!("{}", s)
            },
            BinOp {
                left,
                oper,
                right,
            } => {
                format!("({} {} {})", left, oper, right)
            },
            Null => "Null".to_string(),
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