//! Defines the RUN subcommand.

use carlotk::prelude::*;

const HELP: &str = include_str!("../help_run.txt");

/// Provide help to the user
pub fn helpme() {
    printhelp(HELP);
}

pub fn run(args: CliArgs) {
    if args.contains(Flag::Help) {
        printhelp(HELP);
    }

    let inputfile = args.inputfile.clone();
    let debug = args.contains(Flag::Debug);
    let expressions = parse(inputfile, debug);
    let mut env = Environment::new();
    let output = env.evaluate(&expressions);
    println!("{}", output);
}