use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
    
        println!("\nGuess the number!\nPlease enter your guess: ");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line.");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small.\n"),
            Ordering::Greater => println!("Too big.\n"),
            Ordering::Equal => {
                println!("Correct answer!");
                break;
            }
        }
    }
}
