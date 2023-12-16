use std::ops::Add;
use std::time::Duration;

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    return i + j;
}

fn main() {
    /* Use add() on floats and integers. */
    let num_0 = add(1.2, 3.4);
    let num_1 = add(10, 20);

    /* Use add on a duration. */
    let dur_0 = add(Duration::new(5, 0), Duration::new(10, 0));

    println!("{} {} {:?}", num_0, num_1, dur_0);
}
