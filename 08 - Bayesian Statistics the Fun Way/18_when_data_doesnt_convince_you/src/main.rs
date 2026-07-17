/*
 * Chapter 18 - When Data Doesn’t Convince You
 * ===========================================
 */

fn main() {
    /* Can your friend predict a die roll 90% of the time?  */
    let p_d_predict = 0.9_f64.powi(14) * 0.1;
    let p_d_guess = (1.0_f64 / 6.0_f64).powi(14) * 5.0 / 6.0;

    let bayes_factor = p_d_predict / p_d_guess;

    println!(
        "\nBayes Factor comparing prediction to guessing = {}",
        bayes_factor
    );
}
