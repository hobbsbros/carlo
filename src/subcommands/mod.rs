//! Defines subcommands for the Carlo language.

mod help;
mod repl;
mod latex;
mod run;
mod version;

pub use crate::Error;

pub use help::help;
pub use repl::repl;
pub use latex::latex;
pub use run::run;
pub use version::version;

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
