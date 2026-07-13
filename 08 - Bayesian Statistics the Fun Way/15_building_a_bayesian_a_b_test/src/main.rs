/*
 * Chapter 15 - From Parameter Estimation To Hypothesis Testing: Building A Bayesian A/B Test
 * ==========================================================================================
 */

use rand::distributions::Distribution as OtherDistribution;
use statrs::distribution::{Beta, ContinuousCDF};
use statrs::statistics::Distribution;

fn main() {
    evaluate_a_b_test(
        "DEFAULT PRIOR",
        0.3,
        10.0,
        0.3,
        10.0,
        36.0,
        114.0,
        50.0,
        100.0,
    );
    evaluate_a_b_test(
        "STRONG BALANCE BETWEEN A & B",
        0.3,
        1000.0,
        0.3,
        1000.0,
        36.0,
        114.0,
        50.0,
        100.0,
    );
    evaluate_a_b_test(
        "LOWER PRIOR FOR B",
        0.3,
        10.0,
        0.2,
        10.0,
        36.0,
        114.0,
        50.0,
        100.0,
    );
}

/// Using the data obtained from an A/B test derive evaluation metrics:
///
///     * The success rate of A
///     * The success rate of B
///     * The 95% CI of A
///     * The 95% CI of B
///     * The probability that B is better than A
///     * The magnitude of improvement from moving from B to A
///
fn evaluate_a_b_test(
    test_name: &str,
    prior_prob_a: f64,
    strength_of_prior_a: f64,
    prior_prob_b: f64,
    strength_of_prior_b: f64,
    a_true: f64,
    a_false: f64,
    b_true: f64,
    b_false: f64,
) {
    let num_samples = 10_000;
    let mut sys_rand_src = rand::rngs::OsRng;
    let (true_a_rate, true_b_rate) = (0.25, 0.3);

    /* Setup the two distributions for A and B. */
    let (vari_a, vari_b) = (
        Beta::new(
            a_true + prior_prob_a * strength_of_prior_a,
            a_false + (1.0 - prior_prob_a) * strength_of_prior_a,
        )
        .unwrap(),
        Beta::new(
            b_true + prior_prob_b * strength_of_prior_b,
            b_false + (1.0 - prior_prob_b) * strength_of_prior_b,
        )
        .unwrap(),
    );

    /* Generate samples from each of the distributions. */
    let sample_a: Vec<f64> = vari_a
        .sample_iter(&mut sys_rand_src)
        .take(num_samples)
        .collect();
    let sample_b: Vec<f64> = vari_b
        .sample_iter(&mut sys_rand_src)
        .take(num_samples)
        .collect();

    /* Calculate the probability that B is better than A. */
    let prob_b_better = (0..num_samples)
        .filter(|x| sample_b[*x] > sample_a[*x])
        .count() as f64
        / num_samples as f64;

    /* Determine how much better B is than A. */
    let mag_b_improve = (0..num_samples)
        .map(|x| sample_b[x] / sample_a[x])
        .sum::<f64>()
        / num_samples as f64;

    /* Create simulations of the A/B experiment to determine how much proof is
     * needed to over come the supplied prior. */
    let mut simulu_samples = 0.0;
    loop {
        simulu_samples += 1.0;

        /* Create the distributions. */
        let (simu_a, simu_b) = (
            Beta::new(
                true_a_rate * simulu_samples + prior_prob_a * strength_of_prior_a,
                (1.0 - true_a_rate) * simulu_samples + (1.0 - prior_prob_a) * strength_of_prior_a,
            )
            .unwrap(),
            Beta::new(
                true_b_rate * simulu_samples * strength_of_prior_b,
                (1.0 - true_b_rate) * simulu_samples * strength_of_prior_b,
            )
            .unwrap(),
        );

        /* Generate samples from each of the distributions. */
        let sim_sample_a: Vec<f64> = simu_a
            .sample_iter(&mut sys_rand_src)
            .take(num_samples)
            .collect();
        let sim_sample_b: Vec<f64> = simu_b
            .sample_iter(&mut sys_rand_src)
            .take(num_samples)
            .collect();

        /* Calculate the probability that B is better than A. */
        let prob_b_better_simul = (0..num_samples)
            .filter(|x| sim_sample_b[*x] > sim_sample_a[*x])
            .count() as f64
            / num_samples as f64;

        /* Does this sample size prove b is better? */
        if prob_b_better_simul > 0.95 {
            break;
        }
    }

    println!(
        concat!(
            "\n{}",
            "\n{}",
            "\nP(click through | Variant A) = {:.3}",
            "\nP(click through | Variant B) = {:.3}",
            "\nVariant A 95% CI = {:.3} -> {:.3}",
            "\nVariant B 95% CI = {:.3} -> {:.3}",
            "\nP(B is superior to A) = {:.3}",
            "\nAverage change from A to B = {:.2}",
            "\nSample size required to prove B is better = {}",
        ),
        test_name,
        std::iter::repeat('=')
            .take(test_name.len())
            .collect::<String>(),
        vari_a.mean().unwrap(),
        vari_b.mean().unwrap(),
        vari_a.inverse_cdf(0.025),
        vari_a.inverse_cdf(0.975),
        vari_b.inverse_cdf(0.025),
        vari_b.inverse_cdf(0.975),
        prob_b_better,
        mag_b_improve,
        simulu_samples,
    );
}
