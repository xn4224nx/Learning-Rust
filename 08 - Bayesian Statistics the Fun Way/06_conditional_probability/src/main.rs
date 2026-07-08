/*
 * Chapter 6 - Conditional Probability
 * ===================================
 */

fn main() {
    let p_color_blind: f64 = 0.0425;
    let p_color_blind_male: f64 = 0.08;
    let p_color_blind_female: f64 = 0.005;
    let p_male: f64 = 0.5;

    println!(
        concat!(
            "\nSimplistic Estimation:",
            "\n\tP(male, color blind)   = {:.04}",
            "\n\tP(female, color blind) = {:.04}",
            "\n\nConditional Probability:",
            "\n\tP(male, color blind)   = {:.04}",
            "\n\tP(female, color blind) = {:.04}",
        ),
        p_male * p_color_blind,
        p_male * p_color_blind,
        p_male * p_color_blind_male,
        p_male * p_color_blind_female
    );

    println!(
        concat!(
            "\nBayes' Theorem:",
            "\n\tP(male | color blind)   = {:.04}",
            "\n\tP(female | color blind) = {:.04}",
        ),
        p_male * p_color_blind_male / p_color_blind,
        p_male * p_color_blind_female / p_color_blind,
    );

    println!(
        concat!(
            "\nExercises:",
            "\n\tP(female, ¬ color blind) = {}",
            "\n\tP(color blind OR GBS  | male, flu) = {}",
        ),
        p_male * (1.0 - p_color_blind_female),
        p_color_blind_male + 0.000_03
    );
}
