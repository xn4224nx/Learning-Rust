fn main() {
    
    // x is defined as imutable
    let x = 5;
    
    // x is shadowed
    let x = x + 1;
    {
        // x is shadowed but this only counts within this scope
        let x = x * 2;
        println!("The value of x is: {x}");     // 12
    }
    
    println!("The value of x is: {x}");     // 6
}
