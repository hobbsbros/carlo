//! Defines the HELP subcommand.

use carlotk::prelude::*;

pub const HELP: &str = include_str!("../help.txt");

pub fn help(_args: CliArgs) {
    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
    println!("Developed by Hobbs Bros.");
    println!();
    println!("{}", HELP);
}