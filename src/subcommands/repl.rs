//! Defines the REPL subcommand.

use crate::prelude::*;

pub fn repl(inputfile: Option<PathBuf>, debug: bool) {
    let mut i = 0;

    println!("{}", "The Carlo Language".truecolor(20, 146, 255).bold());
    println!("Version {}", VERSION);
    println!("Developed by Hobbs Bros.");
    println!();

    // Initialize parser
    let parser = Parser::new(debug);

    // Begin environment
    let mut env = Environment::new();

    // Import file
    if let Some(i) = &inputfile {
        println!("Importing {}", i.display());
        println!();
    }
    let imported = parse(inputfile, debug);

    // Evaluated imported file
    env.evaluate(&imported);

    // Begin REPL
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        // Prompt user
        let readline = match rl.readline(&format!(
            "In[{}]  >> ",
            i,
        )) {
            Ok (r) => {
                let _ = rl.add_history_entry(r.as_str());
                r
            },
            Err (_) => Error::CouldNotReadLine (i).throw(),
        };
        println!();

        // Parse input
        let expr = parser.parse(&readline);
        
        // Evaluate input
        let output = env.evaluate(&expr);

        // Output
        let outputstr = format!("Out[{}] >> {}", i, output);
        println!("{}", outputstr.truecolor(198, 215, 247));

        if output.len() == 0 {
            println!();
        }
        
        i += 1;
    }
}