/*
 * Chapter 13 - Tools Of Parameter Estimation: The Pdf, Cdf, And Quantile Function
 * ===============================================================================
 */

use statrs::distribution::{Beta, ContinuousCDF, Normal};
use statrs::statistics::Distribution;

fn main() {
    let click_dist = Beta::new(300.0, 39700.0).unwrap();
    let (upper_bd, lower_bd) = (0.0085, 0.0065);
    let mean_click_rt = click_dist.mean().unwrap();
    let std_dev_cl_rt = click_dist.std_dev().unwrap();

    println!(
        concat!(
            "\nAverage Click Through Rate = {}",
            "\nStandard Deviation         = {}",
            "\nP({} > x > {})     = {}",
            "\nP({} < x)              = {}",
            "\nP({} > x)              = {}",
        ),
        mean_click_rt,
        std_dev_cl_rt,
        upper_bd,
        lower_bd,
        click_dist.cdf(upper_bd) - click_dist.cdf(lower_bd),
        upper_bd,
        1.0 - click_dist.cdf(upper_bd),
        lower_bd,
        click_dist.cdf(lower_bd),
    );

    println!(
        concat!("\nP(P(x) > 0.975) = {}", "\nP(P(x) < 0.025) = {}",),
        click_dist.inverse_cdf(0.975),
        click_dist.inverse_cdf(0.025),
    );

    let snowfall: Vec<f64> = vec![7.8, 9.4, 10.0, 7.9, 9.4, 7.0, 7.0, 7.1, 8.9, 7.4];
    let mean_snowfall = snowfall.iter().sum::<f64>() / snowfall.len() as f64;
    let std_dev_snowfall = (snowfall
        .iter()
        .map(|x| (x - mean_snowfall) * (x - mean_snowfall))
        .sum::<f64>()
        / snowfall.len() as f64)
        .sqrt();
    let snow_dist = Normal::new(mean_snowfall, std_dev_snowfall).unwrap();

    println!(
        concat!(
            "\nMean Snowfall     = {:.2}",
            "\nStd Dev Snowfall  = {:.2}",
            "\nSnowfall 99.9% CI = {:.2} -> {:.2}",
        ),
        mean_snowfall,
        std_dev_snowfall,
        snow_dist.inverse_cdf(0.0005),
        snow_dist.inverse_cdf(0.9995)
    );

    let sell_histr = Beta::new(10.0, 20.0).unwrap();
    let uppr_sell_rate = sell_histr.inverse_cdf(0.975);
    let lowr_sell_rate = sell_histr.inverse_cdf(0.025);

    println!(
        concat!(
            "\nSell rate 95% CI = {:.2} -> {:.2}",
            "\nSales 95% CI     = {:.2} -> {:.2}",
        ),
        lowr_sell_rate,
        uppr_sell_rate,
        40.0 * lowr_sell_rate,
        40.0 * uppr_sell_rate,
    );
}
