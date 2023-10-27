fn main() {
    /* Match groups and if else expressions must return the same type. */
    let suggested_pet = if true { "bat" } else { "dog" };

    /* There must always be a default option. */
    let suggested_num = match suggested_pet {
        "cat" => 1,
        "bat" => 2,
        "dog" => 3,
        _ => 0,
    };

    println!("{} {}", suggested_pet, suggested_num);
}
