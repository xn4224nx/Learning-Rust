use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    
    // Call the program without arguments
    let mut cmd = Command::cargo_bin("test_for_echo").unwrap();
    
    // Check that it failed
    cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
}
