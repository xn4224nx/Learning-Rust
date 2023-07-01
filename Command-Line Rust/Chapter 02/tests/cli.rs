use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/* Echo output comparison files */
const COMP_FILES: &[&str] = &[
    "tests/expected/echo_output_0.txt",
    "tests/expected/echo_output_1.txt",
    "tests/expected/echo_output_2.txt",
    "tests/expected/echo_output_3.txt",
];

fn run_success(args: &[&str], expected_file: &str) -> TestResult {
    
    /* Read the expected string from file */
    let expected_output = fs::read_to_string(expected_file)?;
    
    /* Initalise the command, add arguments and test for success */
    Command::cargo_bin("test_for_echo")?
                .args(args)
                .assert()
                .success()
                .stdout(expected_output);
     
     return Ok(())
}

#[test]
fn dies_no_args() -> TestResult {
    /* Call the program without arguments */
    let mut cmd = Command::cargo_bin("test_for_echo")?;

    /* Check that it failed */
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    
    return Ok(())
}

#[test]
fn echo_output_0() -> TestResult {
    run_success(&["Hello there"], COMP_FILES[0])
}

#[test]
fn echo_output_1() -> TestResult {
    run_success(&["Hello", "there"], COMP_FILES[1])
}

#[test]
fn echo_output_2() -> TestResult {
    run_success(&["Hello there", "-n"], COMP_FILES[2])
}

#[test]
fn echo_output_3() -> TestResult {
    run_success(&["-n", "Hello there"], COMP_FILES[3])
}

