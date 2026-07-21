/*
 * Chapter 2. Atomics
 * ==================
 *
 * A value is calculated once by a thread and is then available to other
 * threads. This can cause a race as it can be uncertain which thread will set
 * the value. This is not an issue though as the value set will always be the
 * same.
 */

use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    let all_threads: Vec<std::thread::JoinHandle<()>> = (0..10)
        .map(|_| {
            std::thread::spawn(move || {
                println!(
                    "Thread '{:?}' determines the value of X to be {}.",
                    std::thread::current().id(),
                    get_x()
                );
            })
        })
        .collect();

    println!("Hello from the main thread!");

    for thread_obj in all_threads.into_iter() {
        thread_obj.join().unwrap()
    }
}

fn get_x() -> u64 {
    static XVAR: AtomicU64 = AtomicU64::new(0);

    /* Get the value of variable x. */
    let mut xvar = XVAR.load(Relaxed);

    /* Has another thread calculated the value of x? */
    if xvar == 0 {
        xvar = 42;
        XVAR.store(xvar, Relaxed);
    }
    return xvar;
}
