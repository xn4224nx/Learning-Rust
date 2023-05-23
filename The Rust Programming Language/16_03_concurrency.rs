use std::sync::{Arc, Mutex};
use std::thread;


fn main() {
    
    /* Create a Atomic Rc Pointer */
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        
        let counter = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
           let mut num = counter.lock().unwrap();
           *num += 1; 
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
    
    let m = Mutex::new(5);
    println!("m = {:?}", m);
    
    
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    
    println!("m = {:?}", m);
}
