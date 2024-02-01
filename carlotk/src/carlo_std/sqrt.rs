//! Defines the square root function for the Carlo language.

use crate::Expression;

/// Evaluates the square root.
pub fn eval(arguments: &Vec<Expression>) -> Expression {
    if arguments.len() != 1 {
        return Expression::Null;
    }

    if let Expression::Float {
        value,
        kg,
        m,
        s,
        a,
        k,
        mol,
    } = arguments[0] {
        return Expression::Float {
            value: value.sqrt(),
            kg: kg * 0.5,
            m: m * 0.5,
            s: s * 0.5,
            a: a * 0.5,
            k: k * 0.5,
            mol: mol * 0.5,
        };
    } else {
        return Expression::FnCall {
            name: "sqrt".to_string(),
            arguments: arguments.to_owned(),
        };
    }
}

/// Formats a square root.
pub fn format(arguments: &Vec<Expression>) -> String {
    if arguments.len() != 1 {
        return String::new();
    }

    format!("sqrt({})", arguments[0])
}

/// Converts a square root to LaTeX.
pub fn latex(arguments: &Vec<Expression>) -> String {
    if arguments.len() != 1 {
        return String::new();
    }

    format!("\\sqrt{{{}}}", arguments[0])
}