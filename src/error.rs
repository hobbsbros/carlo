//! Handles errors for the Carlo language.

use std::{
    fmt,
    process::exit,
};

pub enum Error<'a> {
    /// Could not recognize subcommand
    UnrecognizedSubcommand (&'a str),

    /// Could not find file
    CouldNotFindFile (&'a str),

    /// Could not read file
    CouldNotReadFile (&'a str),

    /// No input file
    NoInputFile,
}

/// Converts an error into a string.
impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        let string = match self {
            UnrecognizedSubcommand (s) => format!("Did not recognize subcommand: {}", s),
            CouldNotFindFile (s) => format!("Could not locate file: {}", s),
            CouldNotReadFile (s) => format!("Could not read file: {}", s),
            NoInputFile => format!("No input file provided")
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