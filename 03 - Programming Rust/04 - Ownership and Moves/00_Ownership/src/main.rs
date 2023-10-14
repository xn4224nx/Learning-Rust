fn main() {
    
    print_padovan();    
}

fn print_padovan() {

    let mut padovan = vec![1, 1, 1];    // Allocated here
    let pad_len = padovan.len(); 
    
    for i in pad_len..10 {
        let next = padovan[i-pad_len] + padovan[i-pad_len+1];
        padovan.push(next);
    }
    
    println!("P(1..10) = {:?}", padovan);
    // Dropped here
}
