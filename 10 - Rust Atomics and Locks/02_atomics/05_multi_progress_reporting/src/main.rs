/*
 * Chapter 2. Atomics
 * ==================
 *
 * Progress reporting from multiple threads using atomic operations.
 */

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::{scope, sleep};
use std::time::Duration;

const NUM_WORK_THREADS: usize = 4;
const NUM_WORK_ITEMS: usize = 100;

fn main() {
    let num_completed = &AtomicUsize::new(0);

    scope(|s| {
        let work_per_thread = NUM_WORK_ITEMS / NUM_WORK_THREADS;

        /* Process all the items accross multiple threads. */
        for work_thrd_idx in 0..NUM_WORK_THREADS {
            s.spawn(move || {
                for i in 0..work_per_thread {
                    println!("Processing item: {}", work_thrd_idx * work_per_thread + i);
                    sleep(Duration::from_secs(1));
                    num_completed.fetch_add(1, Relaxed);
                }
            });
        }

        /* Run a thread to show the overall progress. */
        loop {
            let progress = num_completed.load(Relaxed);
            println!("Working ... {progress:}/{NUM_WORK_ITEMS:} done.");

            if progress == NUM_WORK_ITEMS {
                break;
            }
            sleep(Duration::from_secs(1));
        }
    });
}
