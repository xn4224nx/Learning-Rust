/*
 * Chapter 3. Memory Ordering
 * ==========================
 *
 * The lack of ordering guarantees around relaxed memory ordering can lead to
 * some theoretical complications when operations depend on each other in a
 * cyclic way.
 */

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::spawn;

static X_VAR: AtomicI32 = AtomicI32::new(37);
static Y_VAR: AtomicI32 = AtomicI32::new(0);

fn main() {
    let a = spawn(|| {
        let x_curr = X_VAR.load(Relaxed);
        Y_VAR.store(x_curr, Relaxed);
    });
    let b = spawn(|| {
        let y_curr = Y_VAR.load(Relaxed);
        X_VAR.store(y_curr, Relaxed);
    });

    a.join().unwrap();
    b.join().unwrap();

    println!("\nx = {}\ny = {}", X_VAR.load(Relaxed), Y_VAR.load(Relaxed));
}
