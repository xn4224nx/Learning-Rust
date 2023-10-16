fn main() {
    
    /* Strings are referenced */
    let string_1 = "somnambulance".to_string();
    let string_2 = string_1;
    
    println!("{}\n", string_2);    
    
    /* Integers are copied not referenced*/
    let num1: i32 = 36;
    let num2 = num1;
    
    println!("{}\n{}\n", num1, num2);
}
