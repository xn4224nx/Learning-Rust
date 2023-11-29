fn main() {
    /* The compiler will infer this type. */
    let a = 10;

    /* Types can be declared by the programmer. */
    let b: i32 = 20;

    /* Declare the type in the number itself. */
    let c = 30i32;
    let d = 40_i32;

    let e = add(add(a, b), add(c, d));

    println!("a + b + c + d = {}", e);
}

fn add(i: i32, j: i32) -> i32 {
    return i + j;
}
