//! Main executable for the Carlo language.

use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::Read,
    path::PathBuf,
};

use colored::*;

use carlotk::{
    CliArgs,
    Error,
    Expression,
    Flag,
    Parser,
    Subcommand,
};

const VERSION: &str = "0.1.0";

const HELP: [(&str, &str); 5] = [
    ("", include_str!("./help.txt")),
    ("run", include_str!("./help_run.txt")),
    ("latex", include_str!("./help_latex.txt")),
    ("help", include_str!("./help_help.txt")),
    ("version", include_str!("./help_version.txt")),
];

fn main() {
    use Flag::*;

    let args = CliArgs::parse();

    match args.subcommand {
        Subcommand::Help => help(&args.argument.unwrap_or(String::new())),
        Subcommand::Run => run(
            args.inputfile.clone(),
            args.contains(Debug),
        ),
        Subcommand::Latex => latex(
            args.inputfile.clone(),
            args.contains(Debug),
        ),
        Subcommand::Version => version(),
    };
}

fn help(argument: &str) {
    let hashmap = HashMap::from(HELP);
    let help = match hashmap.get(&argument) {
        Some (h) => h,
        None => Error::NoHelpAvailable (argument).throw(),
    };

    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
    println!("Developed by Hobbs Bros.");
    println!();
    println!("{}", help);
}

fn run(inputfile: Option<PathBuf>, debug: bool) {
    let expressions = parse(inputfile, debug);
    println!("{:#?}", expressions);
}

fn latex(inputfile: Option<PathBuf>, debug: bool) {
    let expressions = parse(inputfile, debug);
    println!("{:#?}", expressions);
}

fn version() {
    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
}

/// Converts a source file into a list of expressions.
fn parse(inputfile: Option<PathBuf>, debug: bool) -> Vec<Expression> {
    if debug {
        println!("{} running Carlo in debug mode", "(notice)".truecolor(220, 180, 0).bold());
        println!();
    }

    // Read data from input file
    let f = match inputfile {
        Some (i) => i,
        None => Error::<&str>::NoInputFile.throw(),
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

    parser.parse(contents)
}