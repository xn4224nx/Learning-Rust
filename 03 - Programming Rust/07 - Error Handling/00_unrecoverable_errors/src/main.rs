use std::env;

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    return half / crew_size as u64;
}

fn main() {
    /* Set the environmental variable to see a backtrace. */
    env::set_var("RUST_BACKTRACE", "1");

    /* Attempt to divide by zero. */
    println!(
        "An individuals pirates share is £{}",
        pirate_share(100000, 0)
    );
}
