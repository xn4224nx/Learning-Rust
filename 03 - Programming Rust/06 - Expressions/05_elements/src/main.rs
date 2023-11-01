use rand;

fn main() {
    /* A vector with random integers. */
    let vect: [u8; 10] = rand::random();

    /* Print the whole vector. */
    println!("{:?}", vect);

    /* Select particular elements in the vector. */
    println!("First element = {}", vect[0]);
    println!("Last element = {}", vect[vect.len() - 1]);

    /* Select a slice of a vector. */
    let first_half = &vect[0..5];
    let last_half = &vect[5..10];

    println!("First half = {:?}", first_half);
    println!("Last half = {:?}", last_half);

    /* End and begining exclusive ranges */
    let all_but_first = &vect[1..];
    let first_eight = &vect[..8];

    println!("All but the first = {:?}", all_but_first);
    println!("First eight = {:?}", first_eight);

    /* Inclusive ranges */
    let first_eight = &vect[..=7];

    println!("First eight = {:?}", first_eight);
}
