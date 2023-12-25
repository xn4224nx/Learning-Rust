use std::fmt::Debug;

/* Implicitly return a unit type. */
fn report<T: Debug>(item: &T) {
    println!("{:?}", item);
}

/* Explicitly return a unit type. */
fn clear(text: &mut String) -> () {
    *text = String::from("");
}

fn main() {
    let mut mystring = String::from("Test");

    report(&mystring);
    clear(&mut mystring);
    report(&mystring);
}
