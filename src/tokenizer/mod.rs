//! Tokenizer for the Carlo language.

mod token;

pub use token::{
    Token,
    TokenClass,
};

/// Stores a stream of characters.
pub struct Charstream {
    chars: Vec<char>,
    index: usize,
}

impl Charstream {
    /// Constructs a new character stream from an input string.
    pub fn from(input: &str) -> Self {
        Self {
            chars: input.chars().collect::<Vec<char>>(),
            index: 0,
        }
    }

    /// Gets the next token from this character stream.
    pub fn get(&mut self) -> Option<Token> {
        use TokenClass::*;

        let peek = match self.peek() {
            Some (p) => p,
            None => return None,
        };

        let class = match peek {
            'A'..='Z' | 'a'..='z' | '_' => Identifier,
            '=' => Assignment,
            '0'..='9' => Number,
            _ => Unknown,
        };

        let mut value = String::new();

        while let Some (c) = self.peek() {
            self.next();

            if c == ' ' {
                break;
            }

            if ('A'..='Z').contains(&c) && class == Identifier {
                value.push(c);
            } else if ('a'..='z').contains(&c) && class == Identifier {
                value.push(c);
            } else if c == '_' && class == Identifier {
                value.push(c);
            } else if ('0'..'9').contains(&c) && class == Identifier {
                value.push(c);
            } else if c == '=' && class == Assignment {
                value.push(c);
            } else if ('0'..'9').contains(&c) && class == Number {
                value.push(c);
            }
        }

        Some (Token {
            class,
            value,
        })
    }

    /// Peeks at the next character in the character stream.
    pub fn peek(&self) -> Option<char> {
        if self.index < self.chars.len() {
            Some (self.chars[self.index])
        } else {
            None
        }
    }

    /// Gets the next character from the character stream.
    pub fn next(&mut self) -> Option<char> {
        let c = self.peek();
        self.index += 1;

        c
    }
}

#[derive(Debug)]
/// Stores a stream of tokens.
pub struct Tokenstream {
    tokens: Vec<Token>,
    index: usize,
}

impl Tokenstream {
    /// Constructs a new token stream from an input string.
    pub fn from(input: &str) -> Self {
        let mut tokens = Vec::new();

        let mut charstream = Charstream::from(input);

        while let Some (t) = charstream.get() {
            tokens.push(t);
        }

        Self {
            tokens,
            index: 0,
        }
    }

    /// Peeks at the next token in the token stream.
    pub fn peek(&self) -> Option<Token> {
        if self.index < self.tokens.len() {
            Some (self.tokens[self.index].to_owned())
        } else {
            None
        }
    }

    /// Gets the next token from the token stream.
    pub fn next(&mut self) -> Option<Token> {
        let t = self.peek();
        self.index += 1;

        t
    }
}

#[test]
fn test_tokenization() {
    let tokens = Tokenstream::from("hello_world = 3");

    println!("{:#?}", tokens);
}