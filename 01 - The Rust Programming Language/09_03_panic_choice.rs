use std::net::IpAddr;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    
    pub fn new(value: i32) -> Guess {
        
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        
        Guess{value}
    }
    
    // Getter function
    pub fn value(&self) -> i32 {
         return self.value
    }
}

fn main() {
    
    let _home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid.");
    
    loop {
        
        
        println!("Enter your guess:");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        
        let guess: i32 = match guess.trim().parse() {
            // Parse another guess if the user's input doesn't parse 
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Your guess is {guess}.");
        
        break;
    }
    
}
