/*
 * Chapter 1. Basics of Rust Concurrency
 * =====================================
 *
 * A thread can park itself, which puts it to sleep, stopping it from consuming
 * any CPU cycles. Another thread can then unpark the parked thread, waking it
 * up from its nap.
 */

use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread::{park, scope, sleep};
use std::time::Duration;

fn main() {
    let queue = Mutex::new(VecDeque::new());

    /* Consuming Thread */
    scope(|s| {
        let thr = s.spawn(|| {
            loop {
                let f_item = queue.lock().unwrap().pop_front();

                /* If there is an item in the queue, show it. */
                if let Some(f_item) = f_item {
                    dbg!(f_item);

                /* Otherwise park this thread. */
                } else {
                    park();
                }
            }
        });

        /* Producing Thread */
        for i in 0.. {
            queue.lock().unwrap().push_back(i);

            /* Unpark the consuming thread. */
            thr.thread().unpark();

            sleep(Duration::from_secs(1));
        }
    });
}
