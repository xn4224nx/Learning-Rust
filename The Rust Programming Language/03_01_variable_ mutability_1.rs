fn main() {
    
    // Defined as a immutable string
    let spaces = "     ";
    println!("{spaces}");
    
    
    // Shadowed as a immutable integer
    let spaces = spaces.len();
    println!("{spaces}");
}
