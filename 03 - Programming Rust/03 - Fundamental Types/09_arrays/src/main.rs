fn main() {
    
    /* Define some arrays. */
    let cater: [u32; 6] = [1, 2, 4, 7, 11, 16,];
    let tax = ["Animalia", "Arthropoda", "Insecta",];
    
    assert_eq!(cater[2], 4);
    assert_eq!(tax.len(), 3);   
    
    /* Define an array of 10000 bool elements all set to true. */
    let mut sieve = [true; 10_000];
    
    for i in 2..100 {
        
        if sieve[i] {
            let mut j = i * i;
            
            while j < 10_000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    
    assert!(sieve[211]);
    assert!(!sieve[9876]);
    
    /* Sort an array. */
    let mut chaos = [3, 5, 4, 1, 2,];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5,]);
}
