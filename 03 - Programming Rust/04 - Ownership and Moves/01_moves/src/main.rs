fn main() {
    
    /* Create a vector of strings. */
    let noodles = vec![
        "udon".to_string(), 
        "ramen".to_string(), 
        "soba".to_string(),
    ];
    
    /* Change ownership of the noodle vector. */
    let t = noodles;
    let u = t;
    
    /* Show the contents of the noodle vector. */
    println!("\n{:?}\n", u);
    
    /* Can't use `noodles` or `u` here. */
    // println!("{:?}", t);
    // println!("{:?}", noodles);
}
