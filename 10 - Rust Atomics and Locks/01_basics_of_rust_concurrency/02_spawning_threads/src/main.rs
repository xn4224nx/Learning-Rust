/*
 * Chapter 1. Basics of Rust Concurrency
 * =====================================
 *
 * Spawning threads and returning values from them.
 */

fn main() {
    let vals = Vec::from_iter(0..1_000_000);

    /* Create a thread to  perform a task. */
    let calc_total_thrd = std::thread::spawn(move || {
        let tmp_total = vals.iter().sum::<usize>();
        println!(
            "Thread '{:?}' calculates the sum of the {} values is {}.",
            std::thread::current().id(),
            vals.len(),
            tmp_total
        );
        return tmp_total;
    });

    /* Receive the value from the thread. */
    let vals_total = calc_total_thrd.join().unwrap();
    println!("Main thread receives the total value as: {}.", vals_total);
}
