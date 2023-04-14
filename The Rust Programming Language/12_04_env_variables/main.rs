use std::{env, process};
use micro_grep::UserInput;

fn main() {
    
    // Parse the argument values
    let args: Vec<String> = env::args().collect();
    
    // Save the arguments as variables
    let config = UserInput::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    
    // Handle errors from reading the file
    if let Err(err) = micro_grep::run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}


