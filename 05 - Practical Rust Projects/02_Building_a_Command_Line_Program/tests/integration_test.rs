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

#[test]
fn run_with_bad_txt_0() {
    Command::cargo_bin("robo_preacher")
        .expect("binary exists")
        .args(&["DRINK"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Father Jack is that you?!"));
}

#[test]
fn run_with_bad_txt_1() {
    Command::cargo_bin("robo_preacher")
        .expect("binary exists")
        .args(&["FECK"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Father Jack is that you?!"));
}

#[test]
fn run_with_bad_txt_2() {
    Command::cargo_bin("robo_preacher")
        .expect("binary exists")
        .args(&["ARSE"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Father Jack is that you?!"));
}

#[test]
fn run_with_bad_txt_4() {
    Command::cargo_bin("robo_preacher")
        .expect("binary exists")
        .args(&["GIRLS"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Father Jack is that you?!"));
}
