fn main() {
    
    let mut letters = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    
    /* Iterate over the vector. */
    for lett in letters {
        
        println!("{}", lett);
        
        /* Try and add to the vector while iterating through it. */
        letters.push(lett.clone());
    };
}
