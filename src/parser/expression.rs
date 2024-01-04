//! Defines Carlo language expressions.

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
                Float {
                    value: self.oper_value(l_value, r_value),
                    kg: self.oper_unit(l_kg, r_kg),
                    m: self.oper_unit(l_m, r_m),
                    s: self.oper_unit(l_s, r_s),
                    a: self.oper_unit(l_a, r_a),
                    k: self.oper_unit(l_k, r_k),
                    mol: self.oper_unit(l_mol, r_mol),
                }
            } else {
                todo!()
            }
        } else {
            todo!()
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
    pub fn oper_unit(&self, left: &f64, right: &f64) -> f64 {
        use BinaryOperation::*;

        match self {
            Add | Sub => if left == right {
                return *left
            } else {
                todo!()
            },
            Mul => left + right,
            Div => left - right,
        }
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
    }
}

impl Expression {
    /// Simplifies an expression.
    pub fn simplify(&self) -> Self {
        use Expression::*;

        match self {
            BinOp {
                left,
                oper,
                right,
            } => oper.simplify(left, right),
            _ => (*self).to_owned(),
        }
    }
}