fn main() {
    
    /* Binary */   
    let three = 0b11;
    
    /* Octal */
    let thirty = 0o32;
    
    /* Hexadecimal */
    let three_hund = 0x12C;
    
    println!("Base 10: {} {} {}", three, thirty, three_hund);
    println!("Base 2:  {:b} {:b} {:b}", three, thirty, three_hund);
    println!("Base 8:  {:o} {:o} {:o}", three, thirty, three_hund);
    println!("Base 16: {:X} {:X} {:X}", three, thirty, three_hund);
}
