/*
 * Chapter 1. Introduction to Async
 * Multiple Connections Between Server & Client
 */

use std::process::{Command, Output};

fn main() {
    let cmd_output: Output = Command::new("./connection_bin")
        .output()
        .expect("Failed to execute command!");

    /* Show what the command produced. */
    if cmd_output.status.success() {
        println!("{}", String::from_utf8_lossy(&cmd_output.stdout));
    } else {
        eprintln!("{}", String::from_utf8_lossy(&cmd_output.stdout));
    }
}
