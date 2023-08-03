fn main() {
    
    let mut s = String::from("Hello World!");
    let my_string_literal = "hello world";
    
    
    // Find the position of the first space in the string
    let word_idx = first_word_idx(&s);
    
    let s1 = first_word_slice(&s);
    
    println!("'{}'", &s[..word_idx]);
    
    println!("'{}'", s1);
       
    // The function can also work on string literals
    println!("'{}'", first_word_slice(my_string_literal));
    
    // Empty the string, set it to ""
    s.clear();
    
    let a = [1, 2, 3, 4, 5, 6];
    
    let slice = &a[1..3];
    
    assert_eq!(slice, &[2, 3]);
}

fn first_word_idx(s: &String) -> usize {
    
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    return s.len()
}

fn first_word_slice(s: &str) -> &str {
    
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Return a slice from the start to before the first space
            return &s[0..i];
        }
    
    }
    
    // Return the whole string if there are no spaces in it
    return &s[..]
}
