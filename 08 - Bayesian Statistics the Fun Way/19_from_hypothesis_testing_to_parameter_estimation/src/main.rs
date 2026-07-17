/*
 * Chapter 19 - From Hypothesis Testing To Parameter Estimation
 * ============================================================
 */

use statrs::distribution::{Binomial, DiscreteCDF};
use std::fs::File;
use std::io::Write;

fn main() {
    let (num_success, num_fail) = (24, 76);
    let (p_prize_attn, p_prize_punt): (f64, f64) = (0.5, 0.05);
    let p_d_attn = p_prize_attn.powi(num_success) * (1.0 - p_prize_attn).powi(num_fail);
    let p_d_punt = p_prize_punt.powi(num_success) * (1.0 - p_prize_punt).powi(num_fail);

    println!(
        concat!(
            "\nNumber of ducks                  = {}",
            "\nNumber of prizes found           = {}",
            "\nP(Data | P(prize = 0.5))         = {}",
            "\nP(Data | P(prize = 0.05))        = {}",
            "\nBayes Factor                     = {}",
            "\nP(24 >= prizes| P(prize = 0.5))  = {}",
            "\nP(24 >= prizes| P(prize = 0.05)) = {}",
        ),
        num_success + num_fail,
        num_success,
        p_d_attn,
        p_d_punt,
        p_d_punt / p_d_attn,
        Binomial::new(p_prize_attn, (num_success + num_fail) as u64)
            .unwrap()
            .cdf(num_success as u64),
        Binomial::new(p_d_punt, (num_success + num_fail) as u64)
            .unwrap()
            .cdf(num_success as u64),
    );

    /* Generate the bayes factors for the range of probabilities. */
    let mut hyp_range = hypothesis_scan(num_success, num_fail, p_prize_attn);

    /* Write the bayes factors to file. */
    write_to_tsv(&hyp_range, "hyp_1000");

    /* Find the location of the best hypothesis. */
    let mut best_idx = 0;
    let mut highest_factor = f64::MIN;
    for idx in 0..hyp_range.len() {
        if hyp_range[idx].1 > highest_factor {
            highest_factor = hyp_range[idx].1;
            best_idx = idx;
        }
    }

    println!(
        concat!("\nBest Hypothesis      = {}", "\nHighest Bayes Factor = {}",),
        hyp_range[best_idx].0, hyp_range[best_idx].1
    );

    /* Set hypothesis between 0.2 and 0.3 to zero */
    for idx in 0..hyp_range.len() {
        if hyp_range[idx].0 >= 0.2 && hyp_range[idx].0 <= 0.3 {
            hyp_range[idx].1 = 0.0;
        }
    }

    /* Write the bayes factors to file again. */
    write_to_tsv(&hyp_range, "hyp_1000_expert");

    /* Normalise the Bayes factors to probabilities. */
    normalise_hyp_range(&mut hyp_range);

    /* Write the bayes factors to file again. */
    write_to_tsv(&hyp_range, "hyp_1000_expert_norm");

    /* Develop a hypothesis range from a 0.24 alternative. */
    let mut alt_hyp_range = hypothesis_scan(num_success, num_fail, 0.24);
    write_to_tsv(&alt_hyp_range, "alt_hyp_1000");

    /* Find the location of the best hypothesis. */
    let mut best_idx = 0;
    let mut highest_factor = f64::MIN;
    for idx in 0..alt_hyp_range.len() {
        if alt_hyp_range[idx].1 > highest_factor {
            highest_factor = alt_hyp_range[idx].1;
            best_idx = idx;
        }
    }

    println!(
        concat!(
            "\nAlt Best Hypothesis      = {}",
            "\nAlt Highest Bayes Factor = {}",
        ),
        alt_hyp_range[best_idx].0, alt_hyp_range[best_idx].1
    );

    normalise_hyp_range(&mut alt_hyp_range);
    write_to_tsv(&alt_hyp_range, "alt_hyp_1000_norm");
}

fn hypothesis_scan(num_success: i32, num_failures: i32, def_prob: f64) -> Vec<(f64, f64)> {
    let incr = 0.001;
    let mut hypth: f64 = 0.0;
    let mut results = Vec::new();

    /* Determine the bayes factor for each hypothesis. */
    while hypth < 1.0 {
        let bays_factor = (hypth.powi(num_success) * (1.0 - hypth).powi(num_failures))
            / (def_prob.powi(num_success) * (1.0 - def_prob).powi(num_failures));
        results.push((hypth, bays_factor));
        hypth += incr;
    }
    return results;
}

fn write_to_tsv(data: &Vec<(f64, f64)>, file_name: &str) {
    /* Create the outputfile */
    let mut data_file = File::create(format!("./data/{}.tsv", file_name)).unwrap();

    /* Write the results to file. */
    for (hyp, bays_factor) in data.iter() {
        let _ = write!(data_file, "{}\t{}\n", hyp, bays_factor);
    }
}

fn normalise_hyp_range(data: &mut Vec<(f64, f64)>) {
    let total_factor = data.iter().map(|x| x.1).sum::<f64>();
    for idx in 0..data.len() {
        data[idx].1 /= total_factor;
    }
}
