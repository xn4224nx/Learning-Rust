fn main() {
    
    print_padovan();
    
    
    {
        let point = Box::new((0.625, 0.5));     // point allocatedon the heap 
                                                // here
        let label = format!("{:?}", point);     // label allocated here
        
        assert_eq!(label, "(0.625, 0.5)");
        
        println!("\npoint = '{:?}'\nlabel = '{}'", point, label);
        // Both label and point dropped here 
    }    
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
