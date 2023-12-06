use rand;

fn main() {
    
    /* Generate an array of random numbers. */
    let haystack: [u8; 32] = rand::random();
    
    for num in &haystack {
        
        let result = match num {
        
            40 => "Bang on!",
            30 ..= 50 => "close",
            _ => "miss",
        };
        println!("{:>3} {:>8}", num, result);
    }
}
