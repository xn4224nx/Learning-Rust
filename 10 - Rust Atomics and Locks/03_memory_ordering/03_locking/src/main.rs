/*
 * Chapter 3. Memory Ordering
 * ==========================
 *
 * Release and acquire memory ordering are used in a pair to form a
 * happens-before relationship between threads. Release memory ordering applies
 * to store operations, while Acquire memory ordering applies to load
 * operations.
 *
 * A happens-before relationship is formed when an acquire-load operation
 * observes the result of a release-store operation. In this case, the store and
 * everything before it, happened before the load and everything after it.
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
