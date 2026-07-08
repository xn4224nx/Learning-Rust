/*
 * Chapter 7 - Bayes' Theorem With Lego
 * ====================================
 */

fn main() {
    let p_kansas = 105.0 / (105.0 + 114.0);
    let p_kansas_city = 15.0 / (105.0 + 114.0);
    let p_kcity_kansas = 6.0 / 105.0;

    println!("P(Kansas City | Kansas) = {:.02}", p_kcity_kansas * p_kansas / p_kansas_city);

    let p_ace = 3.0 / 51.0;
    let p_black = 26.0 / 51.0;
    let p_black_ace = 2.0 / 3.0;

    println!(
        "P(Ace | Black) = {:.02}",
        p_black_ace  * p_ace / p_black
    );
}
