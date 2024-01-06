//! Defines the RUN subcommand.

use crate::prelude::*;

pub fn run(inputfile: Option<PathBuf>, debug: bool) {
    let expressions = parse(inputfile, debug);
    let mut env = Environment::new();
    let output = env.evaluate(&expressions);
    println!("{}", output);
}