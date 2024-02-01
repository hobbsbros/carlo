//! Defines the RUN subcommand.

use carlotk::prelude::*;

pub fn run(args: CliArgs) {
    let inputfile = args.inputfile.clone();
    let debug = args.contains(Flag::Debug);
    let expressions = parse(inputfile, debug);
    let mut env = Environment::new();
    let output = env.evaluate(&expressions);
    println!("{}", output);
}