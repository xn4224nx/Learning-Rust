use std::env;

fn main() {
    
    // Read the argument values
    let args: Vec<String> = env::args().collect();
    
    // Save the arguments as variables
    let search_query = &args[1];
    let filepath = &args[2];
    
    // Show the arguments
    println!("searching file: \"{}\" for string: \"{}\"", 
                                                    filepath, search_query);
}
