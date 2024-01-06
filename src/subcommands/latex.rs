//! Defines the LATEX subcommand.

use crate::prelude::*;

pub fn latex(inputfile: Option<PathBuf>, debug: bool) {
    let expressions = parse(inputfile, debug);
    let mut env = Environment::new();
    let output = env.latex_evaluate(&expressions);
    println!("{}", output);
}