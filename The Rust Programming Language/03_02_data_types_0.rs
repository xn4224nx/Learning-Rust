fn main() {
    
    // Use of integers
    let guess: u32 = "42".parse().expect("Not a number");
    println!("The number is {guess}.");
    
    // Using floats
    let x = 2.0;    // f64
    let y: f32 = 3.0; // f32
    
    println!("The two floats are {x:.2} and {y}");
    
    // Addition
    let sum = 5 + 10;
    println!("5 + 10 = {sum}");
    
    // Subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {difference}");
    
    // Multiplication
    let product = 4 * 30;
    println!("4 * 30 = {product}");
    
    // Division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; 
    println!("56.7 / 32.2 = {quotient}");
    println!("-5 / 3 = {truncated}");
    
    // Remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {remainder}");
    
    // Boolean
    let t = true;
    let f: bool = false;
    println!("{t} {f}");
    
    // Characters
    let c = 'z';
    let z: char = '@';
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");
}
