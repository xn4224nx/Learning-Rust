fn main() {
    let nums = vec![4, 8, 19, 27, 34, 10];
    
    {
        let r = &nums;
        println!("First element in `nums` = {}", r[0]);
        
        /* The reference `r` is dropped here as it goes out of scope. 
        Thus allowing the move to be made without any dangling references. */
    }
    
    let aside = nums;
    println!("First element in `aside` = {}", aside[0]);
}
