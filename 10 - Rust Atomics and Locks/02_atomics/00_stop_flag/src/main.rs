/*
 * Chapter 2. Atomics
 * ==================
 *
 * An AtomicBool is used for a stop flag. Such a flag is used to inform other
 * threads to stop running.
 */

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    /* Spawn a thread to do some work! */
    let background_thread = spawn(|| {
        while !STOP.load(Relaxed) {
            sleep(Duration::from_secs(1));
        }
    });

    /* Listen for user input instrucing the task to end. */
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("Commands: help, stop"),
            "stop" => break,
            cmd_in => println!("Unknown command: '{cmd_in:}'"),
        }
    }

    /* Tell the background thread it needs to stop. */
    STOP.store(true, Relaxed);
    println!("Stop signal sent.");

    /* Wait for the background task to finish. */
    background_thread.join().unwrap();
    println!("Background task halted.");
}
