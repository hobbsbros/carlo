//! Command-line argument parser for the Carlo language.

use std::{
    env::args,
    path::PathBuf,
};

use crate::Error;

#[derive(Clone, Copy, PartialEq, Eq)]
/// Subcommands for the Carlo language executable.
pub enum Subcommand {
    /// Executes a REPL
    Repl,

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
            "repl" => Repl,
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

    /// Runs the Carlo language help menu in interactive mode
    Interactive,
}

/// Converts a string into a flag.
impl From<&str> for Flag {
    fn from(input: &str) -> Self {
        use Flag::*;

        match input {
            "debug" => Debug,
            "interactive" => Interactive,
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
            'i' => Interactive,
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
            Subcommand::Repl
        };

        let mut argument: Option<String> = None;
        let mut inputfile: Option<PathBuf> = None;

        // Parse argument or input file
        if subcommand == Subcommand::Help && args.len() > 2 && !args[2].starts_with("-") {
            argument = Some (args[2].to_owned());
        } else if subcommand != Subcommand::Help && args.len() > 2 {
            inputfile = Some (args[2].as_str().into());
        }

        // Parse flags
        let mut flags = Vec::new();
        let mut i = if let Some (_) = &inputfile {
            3
        } else if let Some(_) = &argument {
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