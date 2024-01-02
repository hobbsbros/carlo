//! Defines Carlo language expressions.

#[derive(Debug)]
pub enum Expression {
    /// Assignment
    Assignment {
        left: String,
        right: Box<Expression>,
    },

    /// 64-bit integer
    Integer (i64),

    /// 64-bit floating-point
    Float (f64),

    /// Identifier
    Identifier (String),
}