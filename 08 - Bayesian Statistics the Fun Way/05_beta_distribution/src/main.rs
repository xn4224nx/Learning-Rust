/*
 * Chapter 5 - The Beta Distribution
 * =================================
 */

use statrs::distribution::{Beta, Binomial, ContinuousCDF, Discrete};

fn main() {
    println!(
        concat!(
            "\nP(data | fair coin)          = {}",
            "\nP(data | unfair coin)        = {}",
            "\nP(coin less that 0.5 | data) = {}"
        ),
        Binomial::new(0.5, 41).unwrap().pmf(14),
        Binomial::new(14.0 / 41.0, 41).unwrap().pmf(14),
        Beta::new(14.0, 27.0).unwrap().cdf(0.5)
    );

    println!(
        concat!(
            "\nP(pull rate < 0.005 | data) = {}",
            "\nP(pull rate > 0.005 | data) = {}",
        ),
        Beta::new(5.0, 1195.0).unwrap().cdf(0.005),
        1.0 - Beta::new(5.0, 1195.0).unwrap().cdf(0.005),
    );

    println!(
        "\nP(P(H) > 0.6 | data) = {}",
        1.0 - Beta::new(4.0, 6.0).unwrap().cdf(0.6)
    );

    let mut exper_1 = Beta::new(9.0, 11.0).unwrap();
    println!(
        "\nP(fair coin | data) = {}",
        exper_1.cdf(0.55) - exper_1.cdf(0.45)
    );

    let mut exper_2 = Beta::new(109.0, 111.0).unwrap();
    println!(
        "\nP(fair coin | data) = {}",
        exper_2.cdf(0.55) - exper_2.cdf(0.45)
    );
}
