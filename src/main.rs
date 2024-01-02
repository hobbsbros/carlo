//! Main executable for the Carlo language.

use std::{
    fs::OpenOptions,
    io::Read,
    path::PathBuf,
};

use carlotk::{
    CliArgs,
    Error,
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
    let f = match inputfile {
        Some (i) => i,
        None => Error::NoInputFile.throw(),
    };

    let strf = format!("{}", f.display());

    let option_file = OpenOptions::new()
        .read(true)
        .open(f);

    let mut file = match option_file {
        Ok (f) => f,
        _ => Error::CouldNotFindFile (&strf).throw(),
    };

    let mut contents = String::new();
    
    match file.read_to_string(&mut contents) {
        Ok (_) => (),
        _ => Error::CouldNotReadFile (&strf).throw(),
    };

    println!("{}", contents);
}

fn version() {
    println!("The Carlo Language, Version {}", VERSION);
}