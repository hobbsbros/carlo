//! Implements behavior of binary operations.

use std::fmt;

use crate::{
    Error,
    Expression,
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
    /// Displays this operation in LaTeX.
    pub fn latex(&self) -> String {
        use BinaryOperation::*;

        let oper = match self {
            Add => " + ",
            Sub => " - ",
            Mul => " ",
            Div => " / ",
        };

        oper.to_string()
    }

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