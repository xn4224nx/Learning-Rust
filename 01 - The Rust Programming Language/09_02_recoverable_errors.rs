use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::error::Error;


fn read_username_from_file() -> Result<String, io::Error> {
    
    // Load the file that has the username
    let username_file_result = File::open("hello.txt");
    
    // Check the file open correctly
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    // Variable to store the username
    let mut username = String::new();   
    
    let ret_username = match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    };
    
    return ret_username
}


fn read_username_from_file2() -> Result<String, io::Error> {
    
    let mut username = String::new();
    
    File::open("hello.txt")?.read_to_string(&mut username)?;
    
    return Ok(username);

}


fn read_username_from_file3() -> Result<String, io::Error> {
    
    return fs::read_to_string("hello.txt");
}


fn last_char_of_first_line(text: &str) -> Option<char> {
    
    return text.lines().next()?.chars().last();
}


fn main() -> Result<(), Box<dyn Error>> {
    
    // Open a file
    let greeting_file_result = File::open("hello.txt");
    
    // Check the file opened correctly
    
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // Matching on different errors
    let greeting_file = match greeting_file_result {
        
        // The file opened fine
        Ok(file) => file,
        
        // Something went wrong
        Err(error) => match error.kind() {
            
            // Create the file if it is not found
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem opening the file: {:?}", e),
            },
            
            // Catch other errors
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        }
    };

    
    // Handle errors without match
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
                
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
       
    
    // Give better error messages on panic
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt file needs to exist");
        
    println!("{:?}", read_username_from_file3().expect("error message"));

    // Other uses for ?
    println!("{:?}", last_char_of_first_line("test_string\n test")
        .expect("No last character"));
    
    
    let greeting_file = File::open("hello.txt")?;
    
    Ok(())
}
