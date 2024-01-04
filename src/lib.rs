//! Main library for the Carlo language.

mod cli;
mod error;
mod expression;
mod parser;
mod tokenizer;
mod unit;

use std::{
    fs::OpenOptions,
    io::{
        Read,
        stdin,
        stdout,
        Write,
    },
    path::PathBuf,
};

use colored::*;

pub use cli::{
    CliArgs,
    Flag,
    Subcommand,
};

pub use error::Error;

pub use expression::{
    BinaryOperation,
    Expression,
};

pub use tokenizer::{
    Token,
    TokenClass,
    Tokenstream,
};

pub use parser::Parser;

pub use unit::{
    PREFIXES,
    UNITS,
};

/// Converts a source file into a list of expressions.
pub fn parse(inputfile: Option<PathBuf>, debug: bool) -> Vec<Expression> {
    if debug {
        println!("{} running Carlo in debug mode", "(notice)".truecolor(220, 180, 0).bold());
        println!();
    }

    // Read data from input file
    let f = match inputfile {
        Some (i) => i,
        None => Error::<&str>::NoInputFile.throw(),
    };

    let strf = format!("{}", f.display());

    let option_file = OpenOptions::new()
        .read(true)
        .open(f);

    let mut file = match option_file {
        Ok (f) => f,
        _ => Error::CouldNotFindFile (&strf).throw(),
    };

    let mut contents = String::new();
    
    match file.read_to_string(&mut contents) {
        Ok (_) => (),
        _ => Error::CouldNotReadFile (&strf).throw(),
    };

    // Construct parser
    let parser = Parser::new(debug);

    parser.parse(&contents)
}

/// Displays a prompt and reads user input.
pub fn read(prompt: &str) -> String {
    let mut buffer = String::new();

    print!("{}", prompt);
    
    match stdout().flush() {
        Ok (_) => (),
        Err (_) => Error::CouldNotFlushStdout (prompt).throw(),
    };
    match stdin().read_line(&mut buffer) {
        Ok (_) => (),
        Err (_) => Error::CouldNotReadLine (prompt).throw(),
    };

    buffer.trim().to_owned()
}