fn print_slice(n: &[f64]) {
    for num in n {
        print!("{} ", num);
    }
    println!();
}

fn main() {
    
    /* Create a vector. */
    let decimal0: Vec<f64> = vec![0.0,  0.707,  1.0,  0.707];
    
    /* Create an array. */
    let decimal1: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    
    print_slice(&decimal0);
    print_slice(&decimal1);
    
    let decimal0_slice: &[f64] = &decimal0;
    let decimal1_slice: &[f64] = &decimal1;
    
    /* Print the first two elements. */
    print_slice(&decimal0[0..2]);
    
    /* Print the elements starting at 2. */
    print_slice(&decimal0[2..]);
    
    /* Print elements 1 and 2. */
    print_slice(&decimal0_slice[1..3]);        
}
