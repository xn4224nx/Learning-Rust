/*
 * Chapter 10 - Introduction To Averaging and Parameter Estimation
 * ===============================================================
 */

fn main() {
    let snow_fall: Vec<f64> = vec![6.2, 4.5, 5.7, 7.6, 5.3, 8.0, 6.9];
    println!(
        "Average Snowfall = {:.2} inches",
        snow_fall.iter().sum::<f64>() / snow_fall.len() as f64
    );

    let normal_temp = 98.6;
    let fever_temp = 100.4;
    let child_temps: Vec<f64> = vec![99.5, 99.6, 99.7, 99.8, 99.9, 100.0];
    let adult_temps: Vec<f64> = vec![97.5, 97.6, 97.7, 97.8, 97.9, 98.0];

    let sys_error = normal_temp - adult_temps.iter().sum::<f64>() / adult_temps.len() as f64;
    let true_child_temp =
        child_temps.iter().map(|x| x + sys_error).sum::<f64>() / child_temps.len() as f64;

    println!(
        concat!(
            "\nSystematic Error = {:.2} F",
            "\nTrue Child Temp  = {:.2} F",
            "\nChild Has Fever  = {}",
        ),
        sys_error,
        true_child_temp,
        true_child_temp >= fever_temp
    );
}
