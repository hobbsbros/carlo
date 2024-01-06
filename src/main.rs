//! Main executable for the Carlo language.

use carlotk::{
    CliArgs,
    Flag,
    Subcommand,
    subcommands::*,
};

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