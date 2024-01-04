//! Tokenizer for the Carlo language.

mod token;

use crate::Error;

pub use token::{
    Token,
    TokenClass,
};

/// Stores a stream of characters.
pub struct Charstream {
    chars: Vec<char>,
    index: usize,
}

/// Specifies characters used to break tokens.
const TOKENBREAK: [char; 3] = [' ', '\n', ';'];

/// Specifies whitespace characters.
const WHITESPACE: [char; 3] = [' ', '\t', '\n'];

impl Charstream {
    /// Constructs a new character stream from an input string.
    pub fn from(input: &str) -> Self {
        Self {
            chars: input.chars().collect::<Vec<char>>(),
            index: 0,
        }
    }

    /// Skip whitespace.
    fn skip(&mut self) {
        if let Some (c) = self.peek() {
            if WHITESPACE.contains(&c) {
                self.next();
            }
        }
    }

    /// Gets the next token from this character stream.
    pub fn get(&mut self) -> Option<Token> {
        use TokenClass::*;

        self.skip();

        let peek = match self.peek() {
            Some (p) => p,
            None => return None,
        };

        let class: TokenClass = peek.into();

        let mut value = String::new();

        while let Some (c) = self.peek() {
            if TOKENBREAK.contains(&c) {
                self.next();
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
            } else if c == '+' && class == Plus {
                value.push(c);
            } else if c == '-' && class == Minus {
                value.push(c);
            } else if c == '*' && class == Times {
                value.push(c);
            } else if c == '/' && class == Divide {
                value.push(c);
            } else if (('0'..='9').contains(&c) || c == '.' || c == 'e' || c == '+' || c == '-') && class == Number {
                value.push(c);
            } else {
                break;
            }

            self.next();
        }

        // Special cases

        // `let` expression
        if value.as_str() == "let" && class == Identifier {
            return Some (Token {
                class: Let,
                value: "let".to_string(),
            });
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
    pub fn from(input: &str, debug: bool) -> Self {
        let mut tokens = Vec::new();

        let mut charstream = Charstream::from(input);

        while let Some (t) = charstream.get() {
            tokens.push(t);
        }

        if debug {
            println!("Constructed token stream with {} tokens", tokens.len());
            println!();
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

    /// Gets the next token and unwraps it, throwing an
    /// unexpected EOF error if no token is available.
    pub fn next_unwrap(&mut self) -> Token {
        let last = match self.tokens.iter().last() {
            Some (t) => t.value.clone(),
            None => String::from("EOF"),
        };

        let next = self.next();

        match next {
            Some (t) => t,
            None => Error::UnexpectedEOF (&last).throw(),
        }
    }

    /// Gets the next token, unwraps it, and returns its value.
    pub fn get(&mut self, class: TokenClass) -> Token {
        let token = self.next_unwrap();

        if token.class == class {
            token
        } else {
            Error::Expected (class, token.class).throw()
        }
    }

    /// Gets the precedence of the next token.
    pub fn precedence(&self) -> u8 {
        match self.peek() {
            Some (t) => t.precedence(),
            None => 0,
        }
    }
}

#[test]
fn test_tokenization() {
    let tokens = Tokenstream::from("hello_world = 3", false);

    println!("{:#?}", tokens);
}