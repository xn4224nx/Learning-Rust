fn main() {
    /* A string stored on the stack. */
    let stack_str = "password123";

    /* A string stored on the heap. */
    let heap_str = String::from("mypassword");

    println!("Is '{}' strong? - {}", &heap_str, is_strong(&heap_str));
    println!("Is '{}' strong? - {}", &stack_str, is_strong(&stack_str));
}

fn is_strong<T: AsRef<str>>(password: T) -> bool {
    return password.as_ref().len() > 5;
}
