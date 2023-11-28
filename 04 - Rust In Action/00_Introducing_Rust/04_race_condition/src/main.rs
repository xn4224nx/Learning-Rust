use std::thread;

fn main() {
    let mut data = 100;

    /* Change the data in two different places. */
    thread::spawn(|| {
        data = 500;
    });
    thread::spawn(|| {
        data = 1000;
    });

    println!("{}", data);
}
