fn main() {
    
    /* Build a vector of number strings. */
    let mut nums = Vec::new();
    
    for i in 0..=105 {
        nums.push(i.to_string());
    }
    
    /* Borrwoing values from the vector. */
    let third = &nums[2];
    let fourth = &nums[3];
    
    println!("\n3rd = {}\n4th = {}\n", third, fourth);
}
