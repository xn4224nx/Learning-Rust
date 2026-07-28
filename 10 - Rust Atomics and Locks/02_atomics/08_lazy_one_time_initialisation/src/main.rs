/*
 * Chapter 2. Atomics
 * ==================
 *
 * Previously we made a function that lazily initializes a value on the first
 * call, but reuses it on later calls.
 *
 * When multiple threads run the function concurrently during the first call,
 * more than one thread might execute the initialization, and they will
 * overwrite each others’ result in an unpredictable order.
 *
 * There are also use cases where such a value gets initialized to a different
 * value each time, even though we need every invocation of the function within
 * a single run of the program to return the same value.
 */

use rand::RngExt;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::scope;

fn main() {
    scope(|s| {
        for thrd_idx in 0..100 {
            s.spawn(move || {
                println!("{}", get_key());
            });
        }
    });
}

fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);

    let key = KEY.load(Relaxed);

    /* Generate a brand new key */
    return if key == 0 {
        let mut rng = rand::rng();
        let new_key = rng.random();

        /* Overwrite with the generated key but only if it is still zero. */
        match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
            Ok(_) => new_key,
            Err(k) => k,
        }

    /* Otherwise just return the key that has been previously been created. */
    } else {
        key
    };
}
