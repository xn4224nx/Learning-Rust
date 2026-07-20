/*
 * Chapter 1. Basics of Rust Concurrency
 * =====================================
 *
 * Threads in Rust
 */

fn main() {
    println!("Spawning the threads");
    std::thread::spawn(my_thread);
    std::thread::spawn(my_thread);
    std::thread::spawn(my_thread);
    println!("Hello from the main thread!");
}

fn my_thread() {
    println!("Hello from thread {:?}", std::thread::current().id());
}
