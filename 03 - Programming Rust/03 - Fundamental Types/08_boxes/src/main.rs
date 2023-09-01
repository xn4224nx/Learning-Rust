fn main() {
    
    let t = (12, "eggs",);
    
    /* Allocate a value on the heap. */
    let b = Box::new(t);
    
    println!("{:?} {:?}", t, b);
}
