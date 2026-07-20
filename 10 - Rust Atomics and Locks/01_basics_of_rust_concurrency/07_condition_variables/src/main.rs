/*
 * Chapter 1. Basics of Rust Concurrency
 * =====================================
 *
 * Threads can wait on a condition variable, after which they can be woken up
 * when another thread notifies that same condition variable. Multiple threads
 * can wait on the same condition variable, and notifications can either be sent
 * to one waiting thread, or to all of them.
 */

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::thread::{scope, sleep};
use std::time::Duration;

fn main() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    scope(|s| {
        s.spawn(|| {
            loop {
                let mut q_ind = queue.lock().unwrap();

                /* Wait to be alerted of an available item. */
                let item = loop {
                    if let Some(item) = q_ind.pop_front() {
                        break item;
                    } else {
                        q_ind = not_empty.wait(q_ind).unwrap();
                    }
                };

                drop(q_ind);
                dbg!(item);
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);

            /* Notify any thread that the queue can be used. */
            not_empty.notify_one();

            sleep(Duration::from_secs(1));
        }
    });
}
