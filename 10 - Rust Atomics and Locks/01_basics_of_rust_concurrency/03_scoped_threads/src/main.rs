/*
 * Chapter 1. Basics of Rust Concurrency
 * =====================================
 *
 * Spawning threads that cannot outlive the closure that is passed, so local
 * variables can safely borrowed.
 */

fn main() {
    let letters = vec!['g', 'a', 't', 't', 'a', 'c', 'a'];

    std::thread::scope(|t| {
        t.spawn(|| {
            println!("Number of letters = {}", letters.len());
        });

        t.spawn(|| {
            for (idx, val) in letters.iter().enumerate() {
                println!("{idx:02}: \"{val:}\"")
            }
        });
    });
}
