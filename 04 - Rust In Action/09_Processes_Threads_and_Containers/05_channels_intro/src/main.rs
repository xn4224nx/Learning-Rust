#[macro_use]
extern crate crossbeam;

use crossbeam::channel::unbounded;
use std::thread;

fn main() {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    /* The select macro simplifies recieves messages. */
    select!(recv(rx) -> msg => println!("{:?}", msg))
}
