//! Main executable for the Carlo language.

use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{
        Read,
        stdin,
        stdout,
        Write,
    },
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

const VERSION: &str = "0.3.0";

const HELP: [(&str, &str); 6] = [
    ("", include_str!("./help.txt")),
    ("repl", include_str!("./help_repl.txt")),
    ("run", include_str!("./help_run.txt")),
    ("latex", include_str!("./help_latex.txt")),
    ("help", include_str!("./help_help.txt")),
    ("version", include_str!("./help_version.txt")),
];

fn main() {
    use Flag::*;
    use Subcommand::*;

    let args = CliArgs::parse();

    match args.subcommand {
        Help => help(
            &args.argument.clone().unwrap_or(String::new()),
            args.contains(Interactive),
        ),
        Repl => repl(
            args.contains(Debug),
        ),
        Run => run(
            args.inputfile.clone(),
            args.contains(Debug),
        ),
        Latex => latex(
            args.inputfile.clone(),
            args.contains(Debug),
        ),
        Version => version(),
    };
}

fn help(argument: &str, interactive: bool) {
    let hashmap = HashMap::from(HELP);

    if interactive {
        println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
        println!("Version {}", VERSION);
        println!("Developed by Hobbs Bros.");
        println!();

        loop {
            let subcommand: &str = &read("help >> ");
            println!();

            if subcommand.trim() == "exit" {
                return;
            }
            
            let help = match hashmap.get(&subcommand.trim()) {
                Some (h) => h,
                None => {
                    println!("Subcommand not recognized");
                    println!();
                    continue;
                }
            };

            println!("{}", help);
            println!();
        }
    } else {
        let help = match hashmap.get(&argument) {
            Some (h) => h,
            None => Error::NoHelpAvailable (argument).throw(),
        };
    
        println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
        println!("Version {}", VERSION);
        println!("Developed by Hobbs Bros.");
        println!();
        println!("{}", help);
        println!();
        println!("Executing `carlo help` will display the help menu.");
    }
}

fn repl(debug: bool) {
    let mut i = 0;

    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
    println!("Developed by Hobbs Bros.");
    println!();

    let parser = Parser::new(debug);

    loop {
        // Prompt user
        let buffer = read(&format!(
            "In[{}]  >> ",
            i,
        ));
        println!();

        // Parse input
        let expr = parser.parse(&buffer);

        // Output
        print!("Out[{}] >> {:#?}", i, expr);
        match stdout().flush() {
            Ok (_) => (),
            Err (_) => Error::CouldNotFlushStdout (i).throw(),
        };
        println!();
        println!();
        
        i += 1;
    }
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

    parser.parse(&contents)
}

/// Reads user input.
fn read(prompt: &str) -> String {
    let mut buffer = String::new();

    print!("{}", prompt);
    
    match stdout().flush() {
        Ok (_) => (),
        Err (_) => Error::CouldNotFlushStdout (prompt).throw(),
    };
    match stdin().read_line(&mut buffer) {
        Ok (_) => (),
        Err (_) => Error::CouldNotReadLine (prompt).throw(),
    };

    buffer
}