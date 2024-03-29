//! Defines Carlo language expressions.

use std::fmt;

use crate::{
    BinaryOperation,
    carlo_std,
    UNITS,
};

/// LaTeX special identifiers
const GREEK: [&str; 13] = [
    "alpha",
    "beta",
    "gamma",
    "delta",
    "epsilon",
    "varepsilon",
    "xi",
    "pi",
    "theta",
    "phi",
    "pi",
    "psi",
    "omega",    
];

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

    /// Full-resolution symbolic identifier
    FullSymbolic (String),

    /// Binary operation
    BinOp {
        left: Box<Expression>,
        oper: BinaryOperation,
        right: Box<Expression>,
    },

    /// Function call
    FnCall {
        name: String,
        arguments: Vec<Expression>,
    },

    /// Subsubheader
    Subsubheader (String),

    /// Subheader
    Subheader (String),

    /// Header
    Header (String),

    /// Paragraph
    Paragraph (String),

    /// Null
    Null,
}

impl Expression {
    pub fn is_numeric(&self) -> bool {
        match self {
            Self::Float {
                value: _,
                kg: _,
                m: _,
                s: _,
                a: _,
                k: _,
                mol: _,
            } => true,
            _ => false,
        }
    }

    pub fn latex(&self, toplevel: bool) -> String {
        use Expression::*;

        let string = match self {
            Assignment {
                left,
                right,
            } => format!("{} := {}", latex_identifier(left), right.latex(true)),
            Reassignment {
                left,
                right,
            } => format!("{} = {}", latex_identifier(left), right.latex(true)),
            Float {
                value,
                kg,
                m,
                s,
                a,
                k,
                mol,
            } => latex_unit(*value, *kg, *m, *s, *a, *k, *mol),
            Identifier (s) => latex_identifier(s),
            Symbolic (s) => format!("{}", s),
            FullSymbolic (s) => format!("{}", s),
            BinOp {
                left,
                oper,
                right,
            } => if toplevel {
                format!("{}{}{}", left.latex(false), oper.latex(), right.latex(false))
            } else {
                format!("({}{}{})", left.latex(false), oper.latex(), right.latex(false))
            },
            FnCall {
                name,
                arguments,
            } => carlo_std::latex(name, arguments),
            Paragraph (s) => format!("\n{}\\par\n", s),
            Header (s) => format!("\n\\section{{{}}}\n", s),
            Subheader (s) => format!("\n\\subsection{{{}}}\n", s),
            Subsubheader (s) => format!("\n\\subsubsection{{{}}}\n", s),
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

    format!("{:.4}{}", value, output)   
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

    format!("{:.4}{}", value, output)
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expression::*;

        let string = match self {
            Assignment {
                left,
                right,
            } => format!("{} = {}", left, right),
            Reassignment {
                left,
                right,
            } => format!("{} = {}", left, right),
            Float {
                value,
                kg,
                m,
                s,
                a,
                k,
                mol,
            } => format_unit(*value, *kg, *m, *s, *a, *k, *mol),
            Identifier (s) => format!("{}", s),
            Symbolic (s) => format!("{}", s),
            FullSymbolic (s) => format!("{}", s),
            BinOp {
                left,
                oper,
                right,
            } => format!("({} {} {})", left, oper, right),
            FnCall {
               name,
               arguments, 
            } => carlo_std::format(name, arguments),
            Paragraph (s) => format!("\n{}\n", s),
            Header (s) => format!("\n{}\n===\n", s.to_uppercase()),
            Subheader (s) => format!("\n{}\n", s.to_uppercase()),
            Subsubheader (s) => format!("\n* {}\n", s),
            Null => "Null".to_string(),
        };

        write!(f, "{}", string)
    }
}

/// Converts an identifier to LaTeX.
fn latex_identifier(id: &str) -> String {
    let mut output = String::new();

    for (i, part) in id.split("_").enumerate() {
        let lower: &str = &part.to_lowercase();
        let part = match GREEK.contains(&lower) {
            true => format!("\\{}", part),
            false => part.to_string(),
        };

        if i == 0 {
            output.push_str(&part);
        } else {
            output.push_str(&format!(
                "_{{{}}}",
                part
            ))
        }
    }

    output
}