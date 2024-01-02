//! Main executable for the Carlo language.

use std::path::PathBuf;

use carlotk::{
    CliArgs,
    Subcommand,
};

const VERSION: &str = "0.1.0";

const HELP: &str = include_str!("./help.txt");

fn main() {
    let args = CliArgs::parse();

    match args.subcommand {
        Subcommand::Help => help(),
        Subcommand::Run => run(args.inputfile),
        Subcommand::Version => version(),
    };
}

fn help() {
    println!("The Carlo Language");
    println!("Version {}", VERSION);
    println!("Developed by Hobbs Bros.");
    println!();
    println!("{}", HELP);
}

fn run(inputfile: Option<PathBuf>) {
    match inputfile {
        Some (i) => println!("Executing {}", i.display()),
        None => println!("No input file provided"),
    }
}

fn version() {
    println!("The Carlo Language, Version {}", VERSION);
}