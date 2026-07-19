/*
 * Chapter 1. Basics of Rust Concurrency
 * =====================================
 */

use std::sync::Arc;
use std::thread;

fn main() {
    let tr1 = thread::spawn(f);
    let tr2 = thread::spawn(f);

    println!("Hello from the main thread!");

    /* Ensure the program does not finish until the threads have completed. */
    tr1.join().unwrap();
    tr2.join().unwrap();

    /* Scoped Threads */
    let numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    thread::scope(|s| {
        s.spawn(|| {
            println!(
                "Thread: {:?} - Length: {}",
                thread::current().id(),
                numbers.len()
            );
        });
        s.spawn(|| {
            for num in numbers.iter() {
                println!("{num:}");
            }
        });
    });

    /* Reference Counting */
    let a = Arc::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let b = a.clone();

    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));

    /* Internal Scope Names */
    let c = Arc::new([
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ]);
    thread::spawn({
        let c = c.clone();
        move || {
            dbg!(c);
        }
    });
    dbg!(c);
}

fn f() {
    println!("Hello from thread id: {:?}", thread::current().id());
}
