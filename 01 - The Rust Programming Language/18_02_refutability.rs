fn main() {
    
    let x = 3;
    let some_option_value: Option<i32> = None;
    
    
    if let Some(x) = some_option_value {
        
        println!("{}", x)
    };
    
    /* Irrefutable if let pattern */
    if let x = 5 {
        
      println!("{}", x);  
    };
}
