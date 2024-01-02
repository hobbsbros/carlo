//! Handles errors for the Carlo language.

use std::{
    fmt,
    process::exit,
};

pub enum Error<'a> {
    /// Could not recognize subcommand
    UnrecognizedSubcommand (&'a str),
}

/// Converts an error into a string.
impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        let string = match self {
            UnrecognizedSubcommand (s) => format!("did not recognize subcommand: {}", s),
        };

        write!(f, "{}", string)
    }
}

impl<'a> Error<'a> {
    pub fn throw(&self) -> ! {
        println!("[ERROR] {}", self);
        exit(0);
    }
}