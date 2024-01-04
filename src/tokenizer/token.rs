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

    /// Gets the precedence of this token.
    pub fn precedence(&self) -> u8 {
        self.class.into()
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

    /// Addition
    Plus,

    /// Subtraction
    Minus,

    /// Multiplication
    Times,

    /// Division
    Divide,

    /// Number
    Number,

    /// Unknown
    Unknown,
}

impl From<TokenClass> for u8 {
    fn from(class: TokenClass) -> Self {
        use TokenClass::*;

        match class {
            Let         => 1,
            Identifier  => 0,
            Assignment  => 1,
            Number      => 0,
            Unknown     => 0,
            Plus        => 2,
            Minus       => 2,
            Times       => 3,
            Divide      => 3,
        }
    }
}

/// Maps characters to classes of tokens that may
/// begin with this character.
impl From<char> for TokenClass {
    fn from(c: char) -> Self {
        use TokenClass::*;

        match c {
            'a'..='z' | 'A'..='Z' | '_' => Identifier,
            '=' => Assignment,
            '+' => Plus,
            '-' => Minus,
            '*' => Times,
            '/' => Divide,
            '0'..='9' => Number,
            _ => Unknown,
        }
    }
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
            Plus => "Plus",
            Minus => "Minus",
            Times => "Times",
            Divide => "Divide",
        };

        write!(f, "{}", string)
    }
}