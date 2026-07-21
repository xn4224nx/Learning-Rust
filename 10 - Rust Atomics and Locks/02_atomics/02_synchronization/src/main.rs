/*
 * Chapter 2. Atomics
 * ==================
 *
 * The main thread now uses park_timeout rather than sleep, such that it can be
 * interrupted.Now, any status updates are immediately reported to the user,
 * while still repeating the last update every second to show that the program
 * is still running.
 */

use std::io::Write;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::{current, park_timeout, scope, sleep};
use std::time::Duration;

const NUM_TASKS: usize = 100;
const MAX_LOADING_DOTS: usize = 5;

fn main() {
    let num_done = AtomicUsize::new(0);
    let main_thrd = current();

    scope(|s| {
        let mut num_loading_dots = 0;

        /* A background thread to process all 100 items. */
        s.spawn(|| {
            for task_idx in 1..=NUM_TASKS {
                sleep(Duration::from_secs(1));
                num_done.store(task_idx, Relaxed);

                /* Wake up the main thread */
                main_thrd.unpark();
            }
        });

        /* The main thread to show status updates. */
        loop {
            num_loading_dots = num_loading_dots % MAX_LOADING_DOTS + 1;
            let curr_progress = num_done.load(Relaxed);

            /* Show the progress to STDOUT */
            print!(
                "\rWorking {curr_progress:}/{NUM_TASKS:} {}{}",
                std::iter::repeat('.')
                    .take(num_loading_dots)
                    .collect::<String>(),
                std::iter::repeat(' ')
                    .take(MAX_LOADING_DOTS - num_loading_dots)
                    .collect::<String>()
            );
            std::io::stdout().flush().unwrap();

            /* Clean up the progress bar. */
            if curr_progress == NUM_TASKS {
                print!("\rAll tasks completed successfully.\n");
                std::io::stdout().flush().unwrap();
                break;
            }

            /* Put this thread into a holding state until the other thread wakes it. */
            park_timeout(Duration::from_millis(500));
        }
    });
}
