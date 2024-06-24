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

#[test]
fn run_with_short_options() {
    Command::cargo_bin("robo_preacher")
        .expect("binary exists")
        .args(&["-e", "-c"])
        .assert()
        .success()
        .stdout(predicate::str::contains("^ ^"));
}

#[test]
fn run_with_long_options() {
    Command::cargo_bin("robo_preacher")
        .expect("binary exists")
        .args(&["--eyes", "--colors"])
        .assert()
        .success()
        .stdout(predicate::str::contains("^ ^"));
}

#[test]
fn run_with_txt_input() {
    Command::cargo_bin("robo_preacher")
        .expect("binary exists")
        .args(&["test string with spaces"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test string with spaces"));
}

#[test]
fn run_with_small_txt_input() {
    Command::cargo_bin("robo_preacher")
        .expect("binary exists")
        .args(&["test_string_without_spaces"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test_string_without_spaces"));
}

#[test]
fn run_with_very_long_input() {
    Command::cargo_bin("robo_preacher")
        .expect("binary exists")
        .args(&["a".repeat(4000)])
        .assert()
        .success()
        .stdout(predicate::str::contains("a"));
}
