//! Defines Carlo language expressions.

#[derive(Clone, Copy, Debug)]
/// Defines binary operations in Carlo.
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
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