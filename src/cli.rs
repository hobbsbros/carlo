//! Command-line argument parser for the Carlo language.

use std::{
    env::args,
    path::PathBuf,
};

use crate::Error;

/// Subcommands for the Carlo language executable.
pub enum Subcommand {
    /// Executes a source file
    Run,

    /// Displays a help menu
    Help,

    /// Displays the version
    Version,
}

/// Converts a string into a subcommand.
impl From<&str> for Subcommand {
    fn from(input: &str) -> Self {
        use Subcommand::*;

        match input {
            "run" => Run,
            "help" => Help,
            "version" => Version,
            _ => Error::UnrecognizedSubcommand (input).throw(),
        }
    }
}

/// Command-line arguments for the Carlo language executable.
pub struct CliArgs {
    /// Executable subcommand
    pub subcommand: Subcommand,

    /// Input file
    pub inputfile: Option<PathBuf>,
}

impl CliArgs {
    /// Parses command-line arguments.
    pub fn parse() -> Self {
        let args = args().collect::<Vec<String>>();

        // Parse subcommand
        let subcommand = if args.len() > 1 {
            args[1].as_str().into()
        } else {
            Subcommand::Help
        };

        // Parse input file
        let inputfile = if args.len() > 2 {
            Some (args[2].as_str().into())
        } else {
            None
        };
    
        Self {
            subcommand,
            inputfile,
        }
    }
}