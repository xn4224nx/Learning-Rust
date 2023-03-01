fn main() {
    
    // Define a mutable string
    let mut str0 = String::from("Hello");
    
    // append a literal to a string
    str0.push_str(" World!");
    
    println!("{}", str0);
    
    // s is now longer valid 
    let str_replacement = str0;
    println!("{}", str_replacement);
    
    // clone is a deep copy of the object in memory
    let str_clone = str_replacement.clone();
    
    // And both are still valid
    println!("s1 = {}, s2 = {}", str_clone, str_replacement);
    
    let str_fn = String::from("Hello");
    
    // The value of str_fn moves into the function and is no longer valid here
    takes_ownership(str_fn);
    
    // str_fn is no longer valid here
    
    // for ints, chars, bool, floats and tuples this rule is not followed
    let x = 5;
    makes_copy(x);
    println!("{}", x);
    
    // The variable is given the ownership from the function
    let returned_str = gives_ownership();
    println!("returned string = \"{}\"", returned_str);
    
    let second_returned_str = takes_and_gives_back(returned_str);
    
    // returned_str is no longer valid
    println!("re-returned string = \"{}\"", second_returned_str);
    
    // Return multiple values from a function
    let (third_returned_str, len) = calculate_length(second_returned_str);
    
    println!("The length of '{}' is {}.", third_returned_str, len)
    
}

fn takes_ownership(some_string: String) {
    
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {

    println!("{}", some_integer)
}

fn gives_ownership() -> String {

    // Create the string
    let some_string = String::from("yours");
    
    // Return the string
    some_string   
}

fn takes_and_gives_back(a_string: String) -> String {
    
    // The function returns the value from a_string
    a_string
}

fn calculate_length(some_string: String) -> (String, usize) {
    
    // Calculate the length of the string
    let length = some_string.len();
    
    // Return a tuple of an int and a String
    (some_string, length)
}





































