/*
 * Chapter 2. Atomics
 * ==================
 *
 * A compare-and-exchange operation checks if the atomic value is equal to a
 * given value, and only if that is the case does it replace it with a new
 * value, all atomically as a single operation.
 *
 * It will return the previous value and tell us whether it replaced it or not.
 */

use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::scope;

fn main() {
    scope(|s| {
        for thrd_idx in 0..=1000 {
            s.spawn(move || {
                println!("Thread: {thrd_idx:03} ID: {:03}", allocate_new_id());
            });
        }
    });
}

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Relaxed);

    loop {
        assert!(id < 1000, "Too many ids!");

        match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
    }
}
