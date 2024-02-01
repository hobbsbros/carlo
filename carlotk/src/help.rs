//! Defines the help utility for the Carlo language.

use std::process::exit;

use colored::*;

use crate::VERSION;

/// Prints help information.
pub fn printhelp(help: &str) -> ! {
    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
    println!("Developed by Hobbs Bros.");
    println!();
    println!("{}", help);
    println!();
    println!("Execute `carlo help` to see the main help menu.");

    exit(0);
}