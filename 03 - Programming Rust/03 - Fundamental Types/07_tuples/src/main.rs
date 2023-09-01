fn main() {
    
    let text = "I see the eigenvalue in thine eye.";
    
    /* Split the string at the 21st char into a tuple of strings. */
    let (head, tail) = text.split_at(21);
    
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye.");
}
