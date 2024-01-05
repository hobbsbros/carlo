//! Main executable for the Carlo language.

use std::{
    collections::HashMap,
    path::PathBuf,
};

use colored::*;

use carlotk::{
    CliArgs,
    Environment,
    Error,
    Flag,
    parse,
    Parser,
    read,
    Subcommand,
};

const VERSION: &str = "0.5.1";

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
            args.inputfile.clone(),
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

            if subcommand == "exit" {
                return;
            }
            
            let help = match hashmap.get(&subcommand) {
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

fn repl(inputfile: Option<PathBuf>, debug: bool) {
    let mut i = 0;

    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
    println!("Developed by Hobbs Bros.");
    println!();

    let parser = Parser::new(debug);
    let mut env = Environment::new();

    // Import
    let imported = parse(inputfile, debug);

    env.evaluate(&imported);

    loop {
        // Prompt user
        let buffer = read(&format!(
            "In[{}]  >> ",
            i,
        ));
        println!();

        // Parse input
        let expr = parser.parse(&buffer);
        
        // Evaluate input
        let output = env.evaluate(&expr);

        // Output
        println!("Out[{}] >> {}", i, output);

        if output.len() == 0 {
            println!();
        }
        
        i += 1;
    }
}

fn run(inputfile: Option<PathBuf>, debug: bool) {
    let expressions = parse(inputfile, debug);
    let mut env = Environment::new();
    let output = env.evaluate(&expressions);
    println!("{}", output);
}

fn latex(inputfile: Option<PathBuf>, debug: bool) {
    let expressions = parse(inputfile, debug);
    let mut env = Environment::new();
    let output = env.latex_evaluate(&expressions);
    println!("{}", output);
}

fn version() {
    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
}