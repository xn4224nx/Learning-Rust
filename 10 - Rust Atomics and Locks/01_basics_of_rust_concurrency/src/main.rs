/*
 * Chapter 1. Basics of Rust Concurrency
 * =====================================
 */

use std::thread;

fn main() {
    let tr1 = thread::spawn(f);
    let tr2 = thread::spawn(f);

    println!("Hello from the main thread!");

    /* Ensure the program does not finish until the threads have completed. */
    tr1.join().unwrap();
    tr2.join().unwrap();
}

fn f() {
    println!("Hello from thread id: {:?}", thread::current().id());
}
