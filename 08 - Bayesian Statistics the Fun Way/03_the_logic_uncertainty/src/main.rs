/*
 * Chapter 3 - The Logic of Uncertainty
 * ====================================
 */

fn main() {
    println!("P(late) = P(train late) x P(bus late) = {:.3}", 0.15 * 0.2);

    println!(
        "P(missing either) = P(miss reg) + P(miss ins) - P(miss reg, miss ins) = {:.3}",
        0.8 + 0.3 - 0.8 * 0.3
    );

    println!("P(D20, D20, D20) = {:.8}", 1.0 / (20.0 * 20.0 * 20.0));

    println!("P(rain, no umbrella) = {:.3}", 0.1 * 0.5);

    println!(
        "P(eat egg salmonella) = {:.8}",
        (1.0 / 20_000.0) + (1.0 / 20_000.0) - (1.0 / 20_000.0) * (1.0 / 20_000.0)
    );

    let prob_hh = 0.5 * 0.5;
    let prob_666 = 1.0 / (6.0 * 6.0 * 6.0);

    println!(
        "P(two heads OR three sixes) = {:.3}",
        prob_hh + prob_666 - prob_hh * prob_666
    );
}
