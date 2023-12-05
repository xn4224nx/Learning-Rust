use std::time::{Duration, Instant};

fn main() {
    let mut cnt = 0;
    
    /* Create a duration that represents one second. */
    let time_limit = Duration::new(1, 0);
    
    /* Accesses the system clock to determine the time. */
    let start = Instant::now();
    
    /* Count how many operations can be performed in one second. */
    while (Instant::now() - start) < time_limit {
        cnt += 1;
    }
    
    println!("{}", cnt);
}
