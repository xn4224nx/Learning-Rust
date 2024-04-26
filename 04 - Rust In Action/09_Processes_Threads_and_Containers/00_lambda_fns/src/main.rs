fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    /* Define the anonymous function. */
    let lambda_add = |a, b| a + b;

    assert_eq!(add(4, 5), lambda_add(4, 5));
}
