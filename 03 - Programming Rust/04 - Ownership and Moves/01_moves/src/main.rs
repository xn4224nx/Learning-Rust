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
    println!("\nu = {:?}\n", u);
    
    /* Can't use `noodles` or `u` here. */
    // println!("{:?}", t);
    // println!("{:?}", noodles);
    
    /* Operations That Move */
    let mut name = "Govinda".to_string();
    name = "Siddhartha".to_string();
    
    /* The "Govinda" has been overwritten. */
    println!("name = {}\n", name);
    
    /* Move without dropping. */
    let mut name2 = "Govinda".to_string();
    let name2_t = name2;
    name2 = "Siddhartha".to_string();
    
    println!("name2_t = {}\n", name2_t);
    println!("name2 = {}\n", name2);
}
