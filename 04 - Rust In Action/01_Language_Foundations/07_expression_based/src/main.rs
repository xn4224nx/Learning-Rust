fn is_even(n: i32) -> bool {
    return n % 2 == 0;
}

fn main() {
    
    let n = 123456;
    let m = 654321;
    
    /* Assign variables from a condition expression. */
    let description = if is_even(n) {
        "even"
    } else {
        "odd"
    };
    
    println!("{} is {}", n, description);
    
    /* Reach the same result with a match expression. */
    let description = match is_even(m) {
        true => "even",
        false => "odd",
    };
    
    println!("{} is {}", m, description); 
    
    /* Return a value from a loop. */
    let o = loop {
        break 123;
    };
    println!("o = {}", o);
}
