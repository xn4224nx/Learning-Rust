fn main() {
    /* Closures are function like values. */
    let is_odd = |a| a % 2 != 0;
    println!("Is 124122 odd: {}", is_odd(124122));

    /* Types can be specified. */
    let is_even = |x: i32| -> bool { x % 2 == 0 };
    println!("Is 124122 even: {}", is_even(124122));
}
