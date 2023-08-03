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
    println!("{}", gcd(462, 1071));
}
