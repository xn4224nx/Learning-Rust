use std::mem::drop;

fn main() {
    
    /* Allocate values on the heap. */
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);
    
    /* Dereference the values on the heap. */
    let result1 = *a + *b + *c;
    
    /* Free some memory on the heap. */
    drop(a);
    
    let d = Box::new(1);
    let result2 = *b + *c + *d;
    
    println!("{} {}", result1, result2);
}
