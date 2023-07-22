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

/// Create a filename that doesn't exist
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

/// Run the program with an argument and verify the result
fn verified_run(args: &[&str], expected_file: &str) -> TestResult {
    
    /* Read the expected result out of a file */
    let expected_result = fs::read_to_string(expected_file)?;
    
    /* Run the program and verify the result */
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected_result);
   
   return Ok(())
}

/// Run the command with an input file, expected file and arguments
fn verified_run_stdin(input_file: &str, 
    args: &[&str], expected_file: &str) -> TestResult {
    
    /* Read the expected and inputed result out of a file */
    let expected_result = fs::read_to_string(expected_file)?;
    let input = fs::read_to_string(input_file)?;
    
    /* Run the program and verify the result */
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected_result);
   
   return Ok(())
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

#[test]
fn ftest_all() -> TestResult {
    return verified_run(&[BUSTLE], "tests/outputs/the-bustle.txt") 
}

#[test]
fn ftest_b_the_bustle() -> TestResult {
    return verified_run(&[BUSTLE], "tests/outputs/b-the-bustle.txt") 
}

#[test]
fn ftest_empty() -> TestResult {
    return verified_run(&[EMPTY], "tests/outputs/empty.txt") 
}

#[test]
fn ftest_fox() -> TestResult {
    return verified_run(&[FOX], "tests/outputs/fox.txt") 
}

#[test]
fn ftest_n_the_bustle() -> TestResult {
    return verified_run(&[BUSTLE], "tests/outputs/n-the-bustle.txt") 
}

#[test]
fn ftest_spiders() -> TestResult {
    return verified_run(&[SPIDERS], "tests/outputs/spiders.txt") 
}

#[test]
fn ftest_the_bustle() -> TestResult {
    return verified_run(&[BUSTLE], "tests/outputs/the-bustle.txt") 
}

