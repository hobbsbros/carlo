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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Flags for the Carlo language executable.
pub enum Flag {
    /// Runs the Carlo language parser in debug mode
    Debug,
}

/// Converts a string into a flag.
impl From<&str> for Flag {
    fn from(input: &str) -> Self {
        use Flag::*;

        match input {
            "debug" => Debug,
            "d" => Debug,
            _ => Error::UnrecognizedFlag (input).throw(),
        }
    }
}

/// Command-line arguments for the Carlo language executable.
pub struct CliArgs {
    /// Executable subcommand
    pub subcommand: Subcommand,

    /// Input file
    pub inputfile: Option<PathBuf>,

    /// Flags
    pub flags: Vec<Flag>,
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

        // Parse flags
        let mut flags = Vec::new();
        let mut i = if let Some (_) = &inputfile {
            3
        } else {
            2
        };
        while i < args.len() {
            let arg = &args[i];

            let flag: Flag = if arg.starts_with("--") {
                arg[2..].into()
            } else if arg.starts_with("-") {
                arg[1..].into()
            } else {
                Error::UnrecognizedArgument (arg).throw();
            };
            
            flags.push(flag);
            i += 1;
        }
    
        Self {
            subcommand,
            inputfile,
            flags,
        }
    }

    /// Check if a specific flag is contained in these arguments.
    pub fn contains(&self, flag: Flag) -> bool {
        for f in &self.flags {
            if *f == flag {
                return true;
            }
        }

        return false;
    }
}