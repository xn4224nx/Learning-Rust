fn main() {
    
    /* Build a vector of number strings. */
    let mut nums = Vec::new();
    
    for i in 0..=105 {
        nums.push(i.to_string());
    }
    
    /* Borrowing values from the vector. */
    let third = &nums[2];
    let fourth = &nums[3];
    
    println!("\n3rd = {}\n4th = {}\n", third, fourth);
    
    /* Pop a value off the end of a vector. */
    let last = nums.pop().expect("Vector empty!");
    println!("Last value = '{}'", last);
    
    /* Move a value out a vector and replace it with the last value. */
    let second = nums.swap_remove(3);
    println!("second = '{}'\nnums[3] = '{}'\n", second, nums[3]);
    
    /* Swap in another value for the one that is taken out. */
    let third = std::mem::replace(&mut nums[2], "substitute".to_string());
    println!("third = '{}'\nnums[2] = '{}'\n", third, nums[2]);
    
    /* Consume all the elements in the vector. */
    for mut n in nums {
        n.push('!');
        println!("{}", n);
    } 
}
