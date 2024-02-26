fn main() {
    /* 40 is on the stack. */
    let a: i32 = 40;

    /* 50 lives on the heap. */
    let b: Box<i32> = Box::new(60);

    println!("{} + {} = {}", a, b, a + *b);
}
