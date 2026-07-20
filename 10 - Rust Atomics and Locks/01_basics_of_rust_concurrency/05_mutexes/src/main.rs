/*
 * Chapter 1. Basics of Rust Concurrency
 * =====================================
 *
 * Use a Mutex to share a mutable value between threads.
 */

use std::sync::Mutex;
use std::thread::{scope, sleep};
use std::time::Duration;

fn main() {
    let total = Mutex::new(0);

    /* Spawn ten threads. */
    scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = total.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }

                /* Signify that other threads can access the value now. */
                drop(guard);

                /* Pause this thread for a second. */
                sleep(Duration::from_secs(1));
            });
        }
    });

    println!("Total = {}", total.into_inner().unwrap());
}
