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
fn runs_with_args() -> TestResult  {
    /* Call the program with valid arguments */
    let mut cmd = Command::cargo_bin("test_for_echo")?;

    /* Check that it passed with the argumen `hello` */
    cmd.arg("hello").assert().success();
    
    return Ok(())
}

#[test]
fn match_echo_output_0() -> TestResult  {
    /* Read the test string from file */
    let expected = fs::read_to_string(COMP_FILES[0])?;

    /* Run the program */
    let mut cmd = Command::cargo_bin("test_for_echo")?;

    /* Give the program arguments and check the result */
    cmd.arg("Hello there").assert().success().stdout(expected);
    
    return Ok(())
}
