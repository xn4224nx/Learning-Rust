fn main() {
    
    let mut i: i32 = 0;
    
    /* While Loop */
    println!("\nWhile Loop");
    
    while i < 4 {
        println!("\ti = {}", i);
        i += 1;
    }
    
    /* While Let Loop */
    println!("\nWhile Let Loop");
    let mut opt_val = Some(0);
    
    while let Some(n) = opt_val {
    
       if n > 9 {
            println!("\tLoop exited."); 
            opt_val = None;
            
       } else {
            println!("\tLoop value = {}", n);
            opt_val = Some(n+1)
       } 
    }
    
    /* Loop Loop */
    println!("\nLoop Loop");
    
    loop {
        println!("\ti = {}", i);
        i += 1;
        if i > 10 {break};
    }
    
    /* For Loop */
    println!("\nFor Loop");
    
    for j in 0..=7 {
        println!("\tj = {}", j);
    }
}
