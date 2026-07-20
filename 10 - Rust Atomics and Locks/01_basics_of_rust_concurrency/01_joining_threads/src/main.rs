/*
 * Chapter 1. Basics of Rust Concurrency
 * =====================================
 *
 * Joining threads.
 */

fn main() {
    let all_threads: Vec<std::thread::JoinHandle<()>> =
        (0..10).map(|_| std::thread::spawn(f)).collect();

    println!("Hello from the main thread!");

    for thread_obj in all_threads.into_iter() {
        thread_obj.join().unwrap()
    }
}

fn f() {
    println!("Hello from thread id: {:?}", std::thread::current().id());
}
