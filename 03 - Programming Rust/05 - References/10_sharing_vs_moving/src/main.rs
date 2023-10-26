fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt)
    }
}

fn main() {
    let nums = vec![4, 8, 19, 27, 34, 10];

    {
        let r = &nums;
        println!("First element in `nums` = {}", r[0]);

        /* The reference `r` is dropped here as it goes out of scope.
        Thus allowing the move to be made without any dangling references. */
    }

    let aside = nums;
    println!("First element in `aside` = {}", aside[0]);

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    /* Concaternate both vectors to `wave`. */
    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    println!("sine wave: {:?}", wave);

    /* Try an append a vector to itself. */
    //extend(&mut wave, &wave);
    //println!("sine wave: {:?}", wave);

    let mut x = 10;

    /* Create two read only references to a variable. */
    let r1 = &x;
    let r2 = &x;

    /* Can't modify after an immutable borrow. */
    //x += 1;

    println!("x   = {}", x);
    println!("*r1 = {}", r1);
    println!("*r2 = {}", r2);

    let mut y = 20;

    /* Create immutable references to a mutable variable. */
    let m1 = &mut y;

    /* modify it using the reference */
    *m1 += 10;

    println!("m1 = {}", *m1);

    /* The ref m1 has been dropped here so an immutable ref can be made. */
    println!("y = {}", y);

    let mut w = (107, 109);

    /* Create an immutable ref */
    let r = &w;

    /* Reborrow the ref */
    let r0 = &r.0;

    /* All the references can be used legally. */
    println!("{} {} {} {} {}", r0, r.0, r.1, w.0, w.1);

    let mut v = (136, 139);
    let m = &mut v;

    /* Reborrowing mutable from mutable. */
    let m0 = &mut m.0;

    *m0 = 137;

    /* Reborrowing shared from mutable and it doesn't overlap with m0. */
    let r1 = &m.1;

    println!("{} {}", r1, m0);
}
