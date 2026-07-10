/*
 * Chapter 11 - Measuring The Spread Of Our Data
 * =============================================
 */

fn main() {
    let well_depth_0: Vec<f64> = vec![3.02, 2.95, 2.98, 3.08, 2.97];
    let well_depth_1: Vec<f64> = vec![3.31, 2.16, 3.02, 3.71, 2.80];
    let well_depth_2: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

    println!(
        concat!(
            "         | dataset 0 | dataset 1 | dataset 2 \n",
            "---------|-----------|-----------|-----------\n",
            "mean     |     {:.3} |     {:.3} |     {:.3} \n",
            "MAD      |     {:.3} |     {:.3} |     {:.3} \n",
            "variance |     {:.3} |     {:.3} |     {:.3} \n",
            "std dev  |     {:.3} |     {:.3} |     {:.3} \n",
        ),
        mean(&well_depth_0),
        mean(&well_depth_1),
        mean(&well_depth_2),
        mean_abs_dev(&well_depth_0),
        mean_abs_dev(&well_depth_1),
        mean_abs_dev(&well_depth_2),
        variance(&well_depth_0),
        variance(&well_depth_1),
        variance(&well_depth_2),
        std_dev(&well_depth_0),
        std_dev(&well_depth_1),
        std_dev(&well_depth_2),
    );
}

fn mean(data: &Vec<f64>) -> f64 {
    return data.iter().sum::<f64>() / data.len() as f64;
}

fn mean_abs_dev(data: &Vec<f64>) -> f64 {
    let data_mean = mean(data);
    return data.iter().map(|x| (x - data_mean).abs()).sum::<f64>() / data.len() as f64;
}

fn variance(data: &Vec<f64>) -> f64 {
    let data_mean = mean(data);
    return (data.iter().map(|x| x * x).sum::<f64>() / data.len() as f64) - data_mean * data_mean;
}

fn std_dev(data: &Vec<f64>) -> f64 {
    return variance(data).sqrt();
}
