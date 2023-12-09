fn main() {
    
    let a = 42;
    
    /* An imutable reference to a.  */
    let r = &a;
    
    /* Use the value r is pointing to.*/
    let b = a + *r;
    
    println!("a + a = {}", b);
}
