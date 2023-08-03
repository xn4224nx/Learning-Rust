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
    
    // Search line by line
    for line in search(&config.search_query, &contents) {
		println!("{line}");
	}

    return Ok(())
}

pub fn search<'a>(search_query: &str, contents: &'a str) -> Vec<&'a str> {
	
	// Storage for the results of the search
	let mut results = Vec::new();
	
	
	for line in contents.lines() {
		if line.contains(search_query ) {
			results.push(line)
		}
	}
	
	return results
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";
		
		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}
}
