//! Main executable for the Carlo language.

use carlotk::{
    CliArgs,
    include_subcommands,
};

fn main() {
    let args = CliArgs::parse();

    include_subcommands!{
        using args

        subcommand run
        subcommand repl
        subcommand help
        subcommand latex
        subcommand version
    };
}