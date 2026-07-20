/*
 * Chapter 2. Atomics
 * ==================
 *
 * Every time the background thread finishes processing an item, it stores the
 * number of processed items in an AtomicUsize. Meanwhile, the main thread shows
 * that number to the user to inform them of the progress, about once per
 * second.
 */

use std::io::Write;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::{scope, sleep};
use std::time::Duration;

const NUM_TASKS: usize = 10;

fn main() {
    let num_done = AtomicUsize::new(0);
    println!("\nTask Processing - {NUM_TASKS:} Items");

    scope(|s| {
        /* A background thread to process all the items. */
        s.spawn(|| {
            for task_idx in 1..=NUM_TASKS {
                sleep(Duration::from_secs(1));

                /* Keep the record of the number completed. */
                num_done.store(task_idx, Relaxed);
            }
        });

        /* The main thread to show status updates. */
        loop {
            let completed = num_done.load(Relaxed);

            print!("\rWorking ... {completed:}/{NUM_TASKS:}"); // \x1Bc for clear screen
            std::io::stdout().flush().unwrap();
            sleep(Duration::from_secs(1));

            if completed == NUM_TASKS {
                break;
            }
        }
    });

    println!("\nTasks Completed");
}
