/*
 * Chapter 1. Introduction to Async
 * Threaded Fibonacci Calculations
 */

const NUM_THREADS: usize = 10;

fn main() {
    let mut threads = Vec::new();

    for thrd_idx in 1..=NUM_THREADS {
        let thrd_handle = std::thread::spawn(move || {
            let result = fibonacci(40);
            println!("Thread {:>2} fib_result: {}", thrd_idx, result);
        });
        threads.push(thrd_handle);
    }

    for thrd_handle in threads {
        thrd_handle.join().unwrap();
    }
}

fn fibonacci(n: u64) -> u64 {
    return if n < 2 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    };
}
