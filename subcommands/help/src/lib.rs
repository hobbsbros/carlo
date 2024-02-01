//! Defines the HELP subcommand.

use std::process::exit;

use carlotk::prelude::*;

pub const HELP: &str = include_str!("../help.txt");
pub const HELP_HELP: &str = include_str!("../help_help.txt");

pub fn help(args: CliArgs) {
    if args.contains(Flag::Help) {
        helpme();
    } else {
        printhelp(HELP);
    }
}

pub fn helpme() {
    printhelp(HELP_HELP);
}

/// Prints help information when the user misuses the help utility.
pub fn helphelp() -> ! {
    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
    println!("Developed by Hobbs Bros.");
    println!();
    println!("Execute `carlo help` to see the main help menu.");

    exit(0);
}