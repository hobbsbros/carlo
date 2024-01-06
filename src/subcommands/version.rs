//! Defines the VERSION subcommand.

use crate::prelude::*;

pub fn version() {
    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
}