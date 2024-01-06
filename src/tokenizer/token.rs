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

    /// Symbolic
    Symbolic,

    /// Opening parenthesis
    OpenParen,

    /// Closing parenthesis
    CloseParen,

    /// Newline
    Newline,

    /// Comment
    Comment,

    /// Unknown
    Unknown,
}

impl From<TokenClass> for u8 {
    fn from(class: TokenClass) -> Self {
        use TokenClass::*;

        match class {
            Let         => 1,
            Identifier  => 0,
            Assignment  => 2,
            Number      => 0,
            Unknown     => 0,
            Plus        => 3,
            Minus       => 3,
            Times       => 4,
            Divide      => 4,
            Symbolic    => 1,
            OpenParen   => 2,
            CloseParen  => 2,
            Newline     => 0,
            Comment     => 0,
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
            '!' => Symbolic,
            '(' => OpenParen,
            ')' => CloseParen,
            '0'..='9' => Number,
            '\n' => Newline,
            '#' => Comment,
            _ => Unknown,
        }
    }
}

impl fmt::Display for TokenClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenClass::*;

        let string = match self {
            Let         => "Let",
            Identifier  => "Identifier",
            Assignment  => "Assignment",
            Number      => "Number",
            Unknown     => "Unknown",
            Plus        => "Plus",
            Minus       => "Minus",
            Times       => "Times",
            Divide      => "Divide",
            Symbolic    => "Symbolic",
            Newline     => "Newline",
            OpenParen   => "OpenParen",
            CloseParen  => "CloseParen",
            Comment     => "Comment",
        };

        write!(f, "{}", string)
    }
}