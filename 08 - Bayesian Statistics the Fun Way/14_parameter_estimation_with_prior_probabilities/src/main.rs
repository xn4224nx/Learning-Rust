/*
 * Chapter 14 - Parameter Estimation With Prior Probabilities
 * ==========================================================
 */

use statrs::distribution::{Beta, ContinuousCDF};
use statrs::statistics::Distribution;

fn main() {
    let (prior_suces, prior_fails) = (1.0, 41.0);
    let (suces, fails) = (25.0, 75.0);
    let (fr_suces, fr_fails) = (86.0, 214.0);

    println!(
        concat!(
            "\nPrior P(click)    = {:.2}",
            "\nData P(click)     = {:.2}",
            "\nCombined P(click) = {:.2}",
            "\nFurther P(click)  = {:.2}",
        ),
        Beta::new(prior_suces, prior_fails).unwrap().mean().unwrap(),
        Beta::new(suces, fails).unwrap().mean().unwrap(),
        Beta::new(prior_suces + suces, prior_fails + fails)
            .unwrap()
            .mean()
            .unwrap(),
        Beta::new(
            prior_suces + suces + fr_suces,
            prior_fails + fails + fr_fails
        )
        .unwrap()
        .mean()
        .unwrap(),
    );

    let sunrises = Beta::new(1.0, (3400.0 + 2026.0) * 365.0).unwrap();
    println!(
        concat!(
            "\nP(the sun will not rise today) = {}",
            "\nPredicted year the sun will not rise = {:.0}\n"
        ),
        sunrises.mean().unwrap(),
        2026.0 + 1.0 / (sunrises.mean().unwrap() * 365.0)
    );

    let dist_names = vec![
        "P(fair coin | weak belief in fairness)",
        "P(fair coin | weak belief in cheating)",
        "P(fair coin | very strong belief in fairness)",
        "P(fair coin | strong belief in cheating)",
    ];
    let dist_priors = vec![(5.0, 5.0), (7.0, 3.0), (1000.0, 1000.0), (70.0, 30.0)];

    for dist_idx in 0..dist_names.len() {
        let dist = Beta::new(9.0 + dist_priors[dist_idx].0, 3.0 + dist_priors[dist_idx].1).unwrap();
        let dist_nxt = Beta::new(
            18.0 + dist_priors[dist_idx].0,
            14.0 + dist_priors[dist_idx].1,
        )
        .unwrap();

        println!(
            "{} = {:.2}\nP(95% CI | prior, more data) = {:.2} -> {:.2}\n",
            dist_names[dist_idx],
            dist.cdf(0.55) - dist.cdf(0.45),
            dist_nxt.inverse_cdf(0.025),
            dist_nxt.inverse_cdf(0.975),
        );
    }
}
