//! Defines Carlo language tokens.

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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
/// Enumerates the classes of Carlo langauge tokens.
pub enum TokenClass {
    /// Identifier
    Identifier,

    /// Assignment operator
    Assignment,

    /// Number
    Number,

    /// Unknown
    Unknown,
}