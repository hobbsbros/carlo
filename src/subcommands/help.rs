//! Defines the HELP subcommand.

use crate::prelude::*;

pub fn help(argument: &str, interactive: bool) {
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