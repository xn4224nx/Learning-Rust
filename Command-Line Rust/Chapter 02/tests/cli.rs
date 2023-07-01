use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

/* Echo output comparison files */
const COMP_FILES: &[&str] = &[
    "tests/expected/echo_output_0.txt",
    "tests/expected/echo_output_1.txt",
    "tests/expected/echo_output_2.txt",
    "tests/expected/echo_output_3.txt",
];

//const COMP_FILE_0: &str = "tests/expected/echo_output_0.txt";


#[test]
fn dies_no_args() {
    
    /* Call the program without arguments */
    let mut cmd = Command::cargo_bin("test_for_echo").unwrap();
    
    /* Check that it failed */
    cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs_with_args() {

    /* Call the program with valid arguments */
    let mut cmd = Command::cargo_bin("test_for_echo").unwrap();
    
    /* Check that it passed with the argumen `hello` */
    cmd.arg("hello").assert().success();
}

#[test]
fn match_echo_output_0() {
    
    /* Read the test string from file */
    let expected = fs::read_to_string(COMP_FILES[0]).unwrap();
    
    /* Run the program */
    let mut cmd = Command::cargo_bin("test_for_echo").unwrap();
    
    /* Give the program arguments and check the result */
    cmd.arg("Hello there").assert().success().stdout(expected);
}
