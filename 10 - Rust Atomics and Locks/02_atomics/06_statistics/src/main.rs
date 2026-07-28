/*
 * Chapter 2. Atomics
 * ==================
 *
 * Collect and report some statistics on the time it takes to process an item.
 */

//use rand::rng;
use rand_distr::{Distribution, Poisson};
use std::io::Write;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::thread::{scope, sleep};
use std::time::{Duration, Instant};

const NUM_WORK_THREADS: usize = 5;
const NUM_WORK_ITEMS: usize = 10000;
const MAX_LOADING_DOTS: usize = 5;

fn main() {
    let num_completed = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);
    let min_time = &AtomicU64::new(0);

    scope(|s| {
        let mut num_loading_dots = 0;
        let work_per_thread = NUM_WORK_ITEMS / NUM_WORK_THREADS;

        /* Threads to do the work. */
        for thrd_idx in 0..NUM_WORK_THREADS {
            s.spawn(move || {
                for wrk_idx in 0..work_per_thread {
                    let start_time = Instant::now();

                    /* Do some sort of work */
                    let poi_disrt =
                        Poisson::new((1 + thrd_idx * work_per_thread + wrk_idx) as f64).unwrap();
                    sleep(Duration::from_micros(
                        poi_disrt.sample(&mut rand::rng()) as u64
                    ));

                    /* How long did the work take? */
                    let time_taken = start_time.elapsed().as_micros() as u64;

                    /* Update the program statistics. */
                    num_completed.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                    min_time.fetch_min(time_taken, Relaxed);
                }
            });
        }

        /* Thread to show status updates. */
        loop {
            num_loading_dots = num_loading_dots % MAX_LOADING_DOTS + 1;
            let elapsed_time = Duration::from_micros(total_time.load(Relaxed));
            let curr_max_time = Duration::from_micros(max_time.load(Relaxed));
            let curr_min_time = Duration::from_micros(min_time.load(Relaxed));
            let curr_completed = num_completed.load(Relaxed);

            print!(
                concat!(
                    "\x1Bc",
                    "\nWorking {}/{} {}{}",
                    "\n\tTotal Task Time:    {:?}",
                    "\n\tMaxiumum Task Time: {:?}",
                    "\n\tMinimum Task Time:  {:?}",
                ),
                curr_completed,
                NUM_WORK_ITEMS,
                std::iter::repeat('.')
                    .take(num_loading_dots)
                    .collect::<String>(),
                std::iter::repeat(' ')
                    .take(MAX_LOADING_DOTS - num_loading_dots)
                    .collect::<String>(),
                elapsed_time,
                curr_max_time,
                curr_min_time
            );
            std::io::stdout().flush().unwrap();

            /* Show the final statistics for the program and then end */
            if curr_completed == NUM_WORK_ITEMS {
                println!("\n\nAll Tasks Completed\n");
                break;
            }
            sleep(Duration::from_millis(500));
        }
    });
}
