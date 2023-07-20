//! Command Line Test Cat Tests

use rand::{distributions::Alphanumeric, Rng};

/// Test Constants
const PRG: &str = "on_the_catwalk";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";


fn create_missing_filename() -> String {
    
    /* Loop until a bad file is found */
    loop {
        
        /* Create a random 7 letter file name */
        let filename: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();
        
        /* If the file doesn't exist return the filename */     
        if fs::metadata(&filename).is_err() {
            return filename
        }
    }
}

