use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn run_with_defaults() {
    Command::cargo_bin("robo_preacher")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "That would be an ecumenical matter!",
        ));
}
