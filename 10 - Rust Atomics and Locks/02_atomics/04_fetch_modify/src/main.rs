/*
 * Chapter 2. Atomics
 * ==================
 *
 * Atomic operations.
 */

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    let val_extrn = AtomicI32::new(1002);

    /* Get the current value and then add 23 to the original. */
    let val_0 = val_extrn.fetch_add(23, Relaxed);
    dbg!(val_0);
    dbg!(&val_extrn);

    /* Determine what the value is. */
    let val_1 = val_extrn.load(Relaxed);
    dbg!(val_1);
    dbg!(&val_extrn);

    /* Get the current value and swap it with another. */
    let val_2 = val_extrn.swap(999, Relaxed);
    dbg!(val_2);
    dbg!(&val_extrn);
}
