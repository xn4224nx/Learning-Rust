fn main() {
    let a: f32 = 42.42;
    
    /* Convert the float to an integer. */
    let franken_val: u32 = unsafe{
        std::mem::transmute(a)
    };
    
    println!("{}", franken_val);
    println!("{:032b}", franken_val);
    
    /* Convert back to a float. */
    let b: f32 = unsafe {
        std::mem::transmute(franken_val)
    };
    
    println!("{}", b);   
    assert_eq!(a, b);
}
