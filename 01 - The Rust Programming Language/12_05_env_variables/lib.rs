use std::{fs, error::Error, env};


pub struct UserInput {
    pub search_query: String,
    pub filepath : String,
    pub ignore_case: bool,
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
		
		// Check the existance of the a environmental variable
		let ignore_case = env::var("IGNORE_CASE").is_ok();
		
		
        return Ok(UserInput {search_query, filepath, ignore_case})
    }
}

pub fn run(config: UserInput) -> Result<(), Box<dyn Error>> {
    
    // Read in the file
    let contents = fs::read_to_string(config.filepath)?;
    
    // Find all the lines that contain the query
    let results = if config.ignore_case {
		search_case_insensitive(&config.search_query, &contents)
	} else {
		search(&config.search_query, &contents)
	};
    
    
    // Search line by line
    for line in results {
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

pub fn search_case_insensitive<'a> (
		search_query: &str, 
		contents: &'a str,
) -> Vec<&'a str> {
	
	let search_query = search_query.to_lowercase();
	
	let mut results = Vec::new();
	
	for line in contents.lines() {
		if line.to_lowercase().contains(&search_query) {
			results.push(line);
		}
	}
	
	return results
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three. 
Duct tape.";
		
		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}
	
	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
	
		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}
}
