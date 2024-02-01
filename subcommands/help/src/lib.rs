//! Defines the HELP subcommand.

use carlotk::prelude::*;

pub const HELP: [(&str, &str); 6] = [
    ("", include_str!("../help.txt")),
    ("repl", include_str!("../../repl/help_repl.txt")),
    ("run", include_str!("../../run/help_run.txt")),
    ("latex", include_str!("../../latex/help_latex.txt")),
    ("help", include_str!("../help_help.txt")),
    ("version", include_str!("../../version/help_version.txt")),
];

pub fn help(args: CliArgs) {
    let interactive = args.contains(Flag::Interactive);
    let argument: &str = &args.argument.unwrap_or(String::new());

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