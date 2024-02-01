//! Defines the VERSION subcommand.

use carlotk::prelude::*;

/// Help menu
const HELP: &str = include_str!("../help_version.txt");

/// Provide help to the user
pub fn helpme() {
    printhelp(HELP);
}

pub fn version(_args: CliArgs) {
    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
}