/*
 * Chapter 2 - Measuring Uncertainty
 * =================================
 */

fn main() {
    println!("P(Sum > 7 with 2 dice) = {}", prob_more_than_7(2));
    println!("P(Sum > 7 with 3 dice) = {}", prob_more_than_7(3));
    println!("P(Red Sox Win)         = {}", convert_odds_to_prob(30, 5));
}

/// Convert odds like 10 to 1 to a probability.
fn convert_odds_to_prob(cost: u32, payout: u32) -> f32 {
    return (payout as f32) / ((cost + payout) as f32);
}

/// What is the probability of getting more than 7 on a number of dice.
fn prob_more_than_7(num_dice: usize) -> f32 {
    return match num_dice {
        0 | 1 => 0.0,
        2 => 0.41,
        3 => 0.84,
        4 => 0.97,
        5 => 0.99,
        6 => 0.99,
        _ => 1.0,
    };
}
