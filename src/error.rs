//! Handles errors for the Carlo language.

use std::{
    fmt,
    process::exit,
};

use colored::*;

/// Enumerates the errors thrown by the Carlo language.
pub enum Error<T: fmt::Display> {
    /// Could not recognize subcommand
    UnrecognizedSubcommand (T),

    /// Could not recognize flag
    UnrecognizedFlag (T),

    /// Could not recognize argument
    UnrecognizedArgument (T),

    /// Could not find file
    CouldNotFindFile (T),

    /// Could not read file
    CouldNotReadFile (T),

    /// No input file
    NoInputFile,

    /// Could not parse number
    CouldNotParseNumber (T),

    /// Could not parse expression
    CouldNotParse (T),

    /// Unexpected EOF
    UnexpectedEOF (T),

    /// Expected
    Expected (T, T),

    /// Could not parse exponent
    CouldNotParseExponent (T),

    /// No help available
    NoHelpAvailable (T),

    /// Could not read REPL line
    CouldNotReadLine (T),

    /// Could not flush stdout
    CouldNotFlushStdout (T),

    /// Undeclared variable
    UndeclaredVariable (T),

    /// Unmatched units
    UnmatchedUnits (T, T, T),
}

/// Converts an error into a string.
impl<T: fmt::Display> fmt::Display for Error<T> {
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
            CouldNotParse (s) => format!("Could not parse near token ({})", s),
            UnexpectedEOF (s) => format!("Unexpected EOF near token ({})", s),
            Expected (x, a) => format!("Expected token of class ({}) but instead found token of class ({})", x, a),
            CouldNotParseExponent (s) => format!("Could not parse as numeric exponent: {}", s),
            NoHelpAvailable (s) => format!("No help available for subcommand: {}", s),
            CouldNotReadLine (i) => format!("Could not read user input near In[{}]", i),
            CouldNotFlushStdout (i) => format!("Could not flust stdout near In[{}]", i),
            UndeclaredVariable (s) => format!("Found undeclared variable: {}", s),
            UnmatchedUnits (u, l, r) => format!("Unmatched unit powers ({}^{}) and ({}^{})", u, l, u, r),
        };

        write!(f, "{}", string)
    }
}

impl<T: fmt::Display> Error<T> {
    pub fn throw(&self) -> ! {
        println!("{} {}", "(error)".truecolor(255, 60, 40).bold(), self);
        exit(0);
    }

    pub fn warn(&self) {
        println!("{} {}\n", "(warn)".truecolor(252, 115, 3).bold(), self);
    }
}