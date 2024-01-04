//! Command-line argument parser for the Carlo language.

use std::{
    env::args,
    path::PathBuf,
};

use crate::Error;

#[derive(Clone, Copy, PartialEq, Eq)]
/// Subcommands for the Carlo language executable.
pub enum Subcommand {
    /// Executes a source file
    Run,

    /// Converts a source file into LaTeX
    Latex,

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
            "latex" => Latex,
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
            _ => Error::UnrecognizedFlag (input).throw(),
        }
    }
}

/// Converts a character into a flag.
impl From<char> for Flag {
    fn from(input: char) -> Self {
        use Flag::*;

        match input {
            'd' => Debug,
            _ => Error::UnrecognizedFlag (input).throw(),
        }
    }
}

/// Command-line arguments for the Carlo language executable.
pub struct CliArgs {
    /// Executable subcommand
    pub subcommand: Subcommand,

    /// Argument to subcommand
    pub argument: Option<String>,

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

        let mut argument = None;

        // Parse argument
        if subcommand == Subcommand::Help && args.len() > 2 {
            argument = Some (args[2].to_owned());
        }

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

            if arg.starts_with("--") {
                flags.push(arg[2..].into());
            } else if arg.starts_with("-") {
                for c in arg[1..].chars() {
                    flags.push(c.into());
                }
            } else {
                Error::UnrecognizedArgument (arg).throw();
            }

            i += 1;
        }
    
        Self {
            subcommand,
            argument,
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