use std::convert::TryInto;

fn main() {
    
    let a: i32 = 10;
    let b: u16 = 100;
    
    if a < (b as i32) {
        println!("Ten is less than one hundred.");
    }
    
    let b_converted = b.try_into().unwrap();
    
    if a < b_converted {
        println!("Ten is less than one hundred.");
    }
}
