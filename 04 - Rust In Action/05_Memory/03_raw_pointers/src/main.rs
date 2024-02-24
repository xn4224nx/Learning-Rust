fn main() {
    let a: i64 = 42;

    /* Cast the ref to variable 'a' to a const raw pointer */
    let a_ptr = &a as *const i64;

    /* Print the value and its address */
    println!("a: {} ({:p})", a, a_ptr);
}
