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

use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::thread::{sleep, spawn};
use std::time::Duration;

static DATA: AtomicU64 = AtomicU64::new(0);
static IS_READY: AtomicBool = AtomicBool::new(false);

fn main() {
    /* Store the data and signify that it is ready. */
    spawn(|| {
        DATA.store(579, Relaxed);
        IS_READY.store(true, Release);
    });

    /* Wait for the data to be ready for use. */
    while !IS_READY.load(Acquire) {
        println!("Waiting ...");
        sleep(Duration::from_millis(100));
    }

    println!("{}", DATA.load(Relaxed));
}
