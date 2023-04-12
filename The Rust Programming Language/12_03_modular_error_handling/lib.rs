use std::{fs, error::Error};


pub struct UserInput {
    pub search_query: String,
    pub filepath : String,
}

impl UserInput {
    pub fn new(args: &[String]) -> Result<UserInput, &'static str> {
        
        // Catch not enough arguments
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        
        // Extract the string to find in the file
        let search_query = args[1].clone();
        
        // Extract the filepath of the file to search
        let filepath = args[2].clone();
    
        return Ok(UserInput {search_query, filepath})
    }
}

pub fn run(config: UserInput) -> Result<(), Box<dyn Error>> {
    // Read in the file
    let contents = fs::read_to_string(config.filepath)?;
    
    println!("File text:\n\n{}\n", contents);
    
    return Ok(())
}
