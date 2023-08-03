fn main() {
    
    let config_max = Some(3u8);
    
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    
    
    let setti_max = Some(4u8);
    
    if let Some(max) = setti_max {
        println!("The maximum is configured to be {}", max);
    }
}
