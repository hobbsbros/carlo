//! Handles errors for the Carlo language.

use std::{
    fmt,
    process::exit,
};

use colored::*;

use crate::TokenClass;

pub enum Error<'a> {
    /// Could not recognize subcommand
    UnrecognizedSubcommand (&'a str),

    /// Could not recognize flag
    UnrecognizedFlag (&'a str),

    /// Could not recognize argument
    UnrecognizedArgument (&'a str),

    /// Could not find file
    CouldNotFindFile (&'a str),

    /// Could not read file
    CouldNotReadFile (&'a str),

    /// No input file
    NoInputFile,

    /// Could not parse number
    CouldNotParseNumber (&'a str),

    /// Could not parse expression
    CouldNotParse (&'a str),

    /// Unexpected EOF
    UnexpectedEOF (&'a str),

    /// Expected
    Expected (TokenClass, TokenClass),
}

/// Converts an error into a string.
impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        let string = match self {
            UnrecognizedSubcommand (s) => format!("Did not recognize subcommand: {}", s),
            UnrecognizedFlag (s) => format!("Did not recognize flag: {}", s),
            UnrecognizedArgument (s) => format!("Did not recognize argument: {}", s),
            CouldNotFindFile (s) => format!("Could not locate file: {}", s),
            CouldNotReadFile (s) => format!("Could not read file: {}", s),
            NoInputFile => format!("No input file provided"),
            CouldNotParseNumber (s) => format!("Could not parse number: {}", s),
            CouldNotParse (s) => format!("Could not parse near token: {}", s),
            UnexpectedEOF (s) => format!("Unexpected EOF near token: {}", s),
            Expected (x, a) => format!("Expected token of class: {} but instead found token of class: {}", x, a),
        };

        write!(f, "{}", string)
    }
}

impl<'a> Error<'a> {
    pub fn throw(&self) -> ! {
        println!("{} {}", "(error)".truecolor(255, 60, 40).bold(), self);
        exit(0);
    }
}