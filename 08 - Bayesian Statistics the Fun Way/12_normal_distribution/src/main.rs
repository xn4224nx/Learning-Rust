/*
 * Chapter 12 - The Normal Distribution
 * ====================================
 */

use statrs::distribution::{ContinuousCDF, Normal};

fn main() {
    let drops_dist = gen_norm_dist(&vec![2.5, 3.0, 3.5, 4.0, 2.0]);

    println!(
        "P(Fuse Time < 18s) = {:.2}",
        gen_norm_dist(&vec![19.0, 22.0, 20.0, 19.0, 23.0]).cdf(18.0)
    );
    println!(
        "P(5sig > mean) = {}",
        1.0 - Normal::new(0.0, 1.0).unwrap().cdf(5.0)
    );
    println!(
        "P(Fever) = {:.2}",
        1.0 - gen_norm_dist(&vec![100.0, 99.8, 101.0, 100.5, 99.7]).cdf(100.4)
    );
    println!(
        "P(Well > 500m) = {}",
        1.0 - drops_dist.cdf(((2.0 * 500.0 / 9.81) as f64).sqrt())
    );
    println!("P(Well < 0m) = {}", drops_dist.cdf(0.0));
}

fn gen_norm_dist(data: &Vec<f64>) -> Normal {
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let std_dev =
        (data.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / data.len() as f64).sqrt();
    return Normal::new(mean, std_dev).unwrap();
}
