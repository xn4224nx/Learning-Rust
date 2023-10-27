use std::env;
use std::process;


fn main() {
    
    /* Take the arguments from the commandline. */
    let prog_args: Vec<String> = env::args().collect();
    
    if prog_args.len() < 2 {
        println!("No argument given to the program!");
        process::exit(1);
    }
    
    /* Attempt to parse the argument to an integer. */
    let unparsed_code = &prog_args[1].parse::<i32>();
    let code = match unparsed_code {
        Ok(value) => value,
        Err(error) => panic!("Error parsing code: {:?}", error),
    };
    
    match code {
        0 => println!("OK"),
        1 => println!("Wires Tangled"),
        2 => println!("User Asleep"),
        _ => println!("Unrecognised Error '{}'", code)
    };
}
