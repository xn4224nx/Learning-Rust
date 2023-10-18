fn main() {
    
    /* x is a standard variable. */
    let x = 10;
    
    /* &x is immutable reference to x */
    let r = &x;
    
    /* *r de-references the r pointer. */
    assert!(*r == 10);
    
    /* Create a mutable value and a reference to it */
    let mut y = 32;
    let m = &mut y;
    
    /* Change the original value through the reference. */
    *m += 32;
    
    assert!(*m == 64);
}
