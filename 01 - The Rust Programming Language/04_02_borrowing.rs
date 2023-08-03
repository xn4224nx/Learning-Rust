fn main() {
    
    let s1 = String::from("hello");    
    let mut mod_str = String::from("hello"); 
    
    let len = calculate_len(&s1);
    
    // To modify it the variable must be mut and the definition as well
    change(&mut mod_str);
    
    println!("The length of '{}' is {}.", s1, len);
    println!("New string = '{mod_str}'");
    
    
    // Only one mutable reference allowed at a time
    // You can have as many immutable references as you want
    {
        // But this works as it is removed at the end of the scope
        let r2 = &mut mod_str;
        println!("{}", r2);
    }
    
    let r1 = &mut mod_str;
    println!("{}", r1);
    
    // Invalid
    //let references_to_nothing = dangle();
}

// The & means a reference to the variable
// It doesn't transfer ownership though and the function can modify it
fn calculate_len(s: &String) -> usize {
    return s.len()
}

fn change(some_str: &mut String) {
    some_str.push_str(", world.");
}

/*
fn dangle() -> &String {
    
    let s = String::from("hello");
    
    return &s
}
*/
