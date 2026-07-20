/*
 * Chapter 1. Basics of Rust Concurrency
 * =====================================
 *
 * Sharing ownership ensures the value is not dropped until there are no owners
 * left.
 */

fn main() {
    let values_0 = std::sync::Arc::new([1, 8, 2, 8, 1, 8, 2, 8, 4]);
    let values_1 = values_0.clone();

    /* Check that both point to the same entity. */
    println!(
        concat!("\nOriginal = {:?}", "\nClone    = {:?}\n",),
        values_0.as_ptr(),
        values_1.as_ptr(),
    );

    /* Use the same name in each thread. */
    std::thread::spawn({
        let values_0 = values_0.clone();
        move || {
            dbg!(values_0);
        }
    })
    .join()
    .unwrap();

    std::thread::spawn({
        let values_1 = values_1.clone();
        move || {
            dbg!(values_1);
        }
    })
    .join()
    .unwrap()
}
