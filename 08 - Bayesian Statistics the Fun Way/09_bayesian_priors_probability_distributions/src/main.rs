/*
 * Chapter 9 - Bayesian Priors And Working With Probability Distributions
 * ======================================================================
 */

use statrs::distribution::{Beta, Continuous, ContinuousCDF};
use statrs::statistics::Distribution;

fn main() {
    let (cpo_survial, cpo_death) = (2.0, 7440.0);
    let (han_survial, han_death) = (20000.0, 1.0);

    println!(
        "P(survival) = {:.03}",
        Beta::new(cpo_survial + han_survial, cpo_death + han_death)
            .unwrap()
            .mean()
            .unwrap()
    );

    let coin_dist = Beta::new(6.0, 1.0).unwrap();
    println!(
        "\nP(0.4 < P(H) < 0.6) = {:.07}",
        coin_dist.cdf(0.6) - coin_dist.cdf(0.4)
    );

    /* What prior would be required to assume given the data the coin is fair. */
    let mut prior = 0.0;
    let (data_h, data_t) = (6.0, 1.0);

    loop {
        prior += 1.0;

        /* Formulate the Beta distribution. */
        let new_dist = Beta::new(data_h + prior, data_t + prior).unwrap();
        let prop_fair = new_dist.cdf(0.6) - new_dist.cdf(0.4);

        /* Does this prior mean a 95% chance the coin is fair. */
        if prop_fair > 0.95 {
            println!("\nPrior to assume fairness = {}", prior);
            break;
        }
    }

    /* How many more heads would be required to overturn the previous prior? */
    let mut new_heads = 0.0;

    loop {
        new_heads += 1.0;

        /* Formulate the Beta distribution. */
        let new_dist = Beta::new(data_h + prior + new_heads, data_t + prior).unwrap();
        let prop_fair = new_dist.cdf(0.6) - new_dist.cdf(0.4);

        if prop_fair < 0.5 {
            println!("\nNew heads to overturn the prior = {}", new_heads);
            break;
        }
    }
}
