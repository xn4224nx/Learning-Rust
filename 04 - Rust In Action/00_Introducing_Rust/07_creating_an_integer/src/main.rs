use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    
    /* Integer on the stack. */
    let a = 10;
    
    /* Integer on the heap. */
    let b = Box::new(20);
    
    /* On the heap within a reference counter. */
    let c = Rc::new(Box::new(30));
    
    /* On the heap within an atomic reference counter and protected by 
    a mutual excusion lock. */
    let d = Arc::new(Mutex::new(40));

    println!("a: {:?}\nb: {:?}\nc: {:?}\nd: {:?}\n", a, b, c, d);
}
