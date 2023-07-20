//! Command Line Test Cat Tests

use rand::{distributions::Alphanumeric, Rng};
use std::fs;
use assert_cmd::Command;
use predicates::prelude::*;

/// Test Constants
const PRG: &str = "on_the_catwalk";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

/// The type returned by all tests
type TestResult = Result<(), Box<dyn std::error::Error>>;



fn create_missing_filename() -> String {
    
    /* Loop until a bad file is found */
    loop {
        
        /* Create a random 7 alphanumeric file name */
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

#[test]
fn skip_bad_file() -> TestResult {

    /* Generate the name of a non existant file */
    let filename = create_missing_filename();
    
    /* The match string to the correct error message */
    let expected = format!("{}: .* [(]os error2[)]", filename);
    
    /* Run the program with a missing file */
    Command::cargo_bin(PRG)?
        .arg(&filename)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    
    return Ok(())
}

