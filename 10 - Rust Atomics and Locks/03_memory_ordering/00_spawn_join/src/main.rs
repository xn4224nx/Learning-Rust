/*
 * Chapter 3. Memory Ordering
 * ==========================
 *
 * Spawning a thread creates a happens-before relationship between what happened
 * before the spawn() call, and the new thread.
 *
 * Similarly, joining a thread creates a happens-before relationship between the
 * joined thread and what happens after the join() call.
 */

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::spawn;

static X_VAR: AtomicI32 = AtomicI32::new(0);

fn main() {
    println!("Main   X_VAR = {}", X_VAR.load(Relaxed));

    X_VAR.store(1, Relaxed);
    println!("Main   X_VAR = {}", X_VAR.load(Relaxed));

    let thrd = spawn(f);

    X_VAR.store(2, Relaxed);
    println!("Main   X_VAR = {}", X_VAR.load(Relaxed));
    thrd.join().unwrap();

    X_VAR.store(3, Relaxed);
    println!("Main   X_VAR = {}", X_VAR.load(Relaxed));
}

fn f() {
    let x_curr = X_VAR.load(Relaxed);

    println!("Thread X_VAR = {x_curr:}");

    /* Will never fail. */
    assert!(x_curr == 1 || x_curr == 2, "This should never been seen!");
}
