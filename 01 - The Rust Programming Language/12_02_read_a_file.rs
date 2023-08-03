use std::{env, fs};

fn main() {
    
    // Read the argument values
    let args: Vec<String> = env::args().collect();
    
    // Save the arguments as variables
    let search_query = &args[1];
    let filepath = &args[2];
    
    // Show the arguments
    println!("File: \"{}\" \nString: \"{}\"", filepath, search_query);
                                                    
    // Read in the file
    let contents = fs::read_to_string(filepath)
                    .expect("Should have been able to read the file");
    
    println!("File text:\n\n{}\n", contents);
}
