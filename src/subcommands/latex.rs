//! Defines the LATEX subcommand.

use crate::prelude::*;

/// LaTeX header
const HEADER: &str = include_str!("../latex_header.tex");

/// LaTeX footer
const FOOTER: &str = include_str!("../latex_footer.tex");

pub fn latex(inputfile: Option<PathBuf>, debug: bool) {
    // Create input method
    let mut rl = DefaultEditor::new().unwrap();

    // Get title
    let title = match rl.readline("Title >> ") {
        Ok (r) => r,
        Err (_) => Error::CouldNotReadLine ("Title").throw(),
    };

    // Get author
    let author = match rl.readline("Author >> ") {
        Ok (r) => r,
        Err (_) => Error::CouldNotReadLine ("Author").throw(),
    };

    let mut output = String::new();

    let mut outputfile = inputfile.clone().unwrap();

    // Output header
    output.push_str(HEADER);

    // Output title and author
    output.push_str(&format!(
        "\\title{{{}}}\n",
        title,
    ));
    output.push_str(&format!(
        "\\author{{{}}}\n",
        author,
    ));
    output.push_str("\\maketitle\n");

    // Parse and evaluate code
    let expressions = parse(inputfile, debug);
    let mut env = Environment::new();
    let latex = env.latex_evaluate(&expressions);

    // Output code
    output.push_str(&latex);

    // Output footer
    output.push_str(FOOTER);

    // Write to output file
    outputfile.set_extension("tex");
    let _ = fs::write(&outputfile, output);

    println!("\nOutput written to {}", outputfile.display());
}