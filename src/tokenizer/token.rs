//! Defines Carlo language tokens.

use std::fmt;

#[derive(Clone, Debug)]
/// A Carlo language token.
pub struct Token {
    pub class: TokenClass,
    pub value: String,
}

impl Token {
    /// Constructs a new token.
    pub fn new(class: TokenClass, value: String) -> Self {
        Self {
            class,
            value,
        }
    }

    /// Checks if this token is of a given class.
    pub fn check(&self, class: TokenClass) -> bool {
        self.class == class
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, \"{}\")", self.class, self.value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
/// Enumerates the classes of Carlo langauge tokens.
pub enum TokenClass {
    /// Let
    Let,

    /// Identifier
    Identifier,

    /// Assignment operator
    Assignment,

    /// Number
    Number,

    /// Unknown
    Unknown,
}

impl fmt::Display for TokenClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenClass::*;

        let string = match self {
            Let => "Let",
            Identifier => "Identifier",
            Assignment => "Assignment",
            Number => "Number",
            Unknown => "Unknown",
        };

        write!(f, "{}", string)
    }
}