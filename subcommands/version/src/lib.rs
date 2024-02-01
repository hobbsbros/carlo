//! Defines the VERSION subcommand.

use carlotk::prelude::*;

pub fn version(_args: CliArgs) {
    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
}