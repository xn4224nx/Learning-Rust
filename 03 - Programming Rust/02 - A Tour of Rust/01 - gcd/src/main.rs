use std::str::FromStr;
use std::env;


fn gcd(mut a: u64, mut b: u64) -> u64 {

    /* Check the input variables */
    assert!(a != 0 && b != 0);
    
    /* Until the remainder is zero. */
    while b != 0 {
        
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    
    return a;
}

fn main() {
    
    /* Vector to store the numbers from the command line. */
    let mut nums = Vec::new();
    
    /* Read the command line arguments */
    for arg in env::args().skip(1) {
    
        /* Append each argument to the nums vector. */
        nums.push(
            u64::from_str(&arg).expect("Error Parsing Argument")
        );
    }
    
    /* Check that enough numbers have been provided. */
    if nums.len() < 2 {
        eprintln!("Usage: gcd NUMBER NUMBER ...");
        std::process::exit(1);
    }
    
    /* Calculate the gcd of all the provided numbers */
    let mut d = nums[0];
    
    for m in &nums[1..] {
        d = gcd(d, *m);
    }
    
    println!("The greatest common divisor of {:?} is {}", nums, d);
}
