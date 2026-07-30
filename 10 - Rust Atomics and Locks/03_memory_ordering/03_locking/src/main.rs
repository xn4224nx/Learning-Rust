/*
 * Chapter 3. Memory Ordering
 * ==========================
 *
 * When locking, they use an atomic operation to check if it was unlocked, using
 * acquire ordering, while also (atomically) changing the state to “locked.”
 * When unlocking, they set the state back to “unlocked” using release ordering.
 */

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread::scope;

static mut DATA: String = String::new();
static IS_LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if IS_LOCKED
        .compare_exchange(false, true, Acquire, Relaxed)
        .is_ok()
    {
        unsafe { DATA.push('!') };
        IS_LOCKED.store(false, Release);
    }
}

fn main() {
    scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });
}
