//! Main executable for the Carlo language.

use std::{
    fs::OpenOptions,
    io::Read,
    path::PathBuf,
};

use colored::*;

use carlotk::{
    CliArgs,
    Error,
    Flag,
    Parser,
    Subcommand,
};

const VERSION: &str = "0.1.0";

const HELP: &str = include_str!("./help.txt");

fn main() {
    use Flag::*;

    let args = CliArgs::parse();

    match args.subcommand {
        Subcommand::Help => help(),
        Subcommand::Run => run(
            args.inputfile.clone(),
            args.contains(Debug),
        ),
        Subcommand::Version => version(),
    };
}

fn help() {
    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
    println!("Developed by Hobbs Bros.");
    println!();
    println!("{}", HELP);
}

fn run(inputfile: Option<PathBuf>, debug: bool) {
    if debug {
        println!("{} running Carlo in debug mode", "(notice)".truecolor(255, 255, 0));
        println!();
    }

    // Read data from input file
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

    // Construct parser
    let parser = Parser::new(debug);

    println!("{:#?}", parser.parse(contents));
}

fn version() {
    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
}