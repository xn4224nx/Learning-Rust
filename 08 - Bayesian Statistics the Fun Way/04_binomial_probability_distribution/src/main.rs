/*
 * Chapter 4 - Creating a Binomial Probability Distribution
 * ========================================================
 */

use statrs::distribution::{Binomial, Discrete, DiscreteCDF};

fn main() {
    println!(
        "P(12 heads from 24 flips) = {}",
        Binomial::new(0.5, 24).unwrap().pmf(12)
    );

    println!(
        "P(pulling one card) = {}",
        Binomial::new(0.0072, 100).unwrap().pmf(1)
    );

    println!(
        "P(pulling at least one card) = {}",
        1.0 - Binomial::new(0.0072, 100).unwrap().cdf(0)
    );

    println!(
        "P(rolling a 1 or 20) = {}",
        1.0 - Binomial::new(0.1, 12).unwrap().pmf(0)
    );

    println!(
        "P(one ace in five pulls) = {}",
        Binomial::new(1.0 / 13.0, 5).unwrap().pmf(1)
    );

    println!(
        "P(five aces in ten pulls) = {}",
        Binomial::new(1.0 / 13.0, 10).unwrap().pmf(5)
    );

    println!(
        "P(At least two job offers) = {}",
        1.0 - Binomial::new(0.2, 7).unwrap().cdf(1)
    );

    println!(
        "P(At least two job offers) = {}",
        1.0 - Binomial::new(0.1, 25).unwrap().cdf(1)
    );
}
