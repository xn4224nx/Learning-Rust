/*
 * Chapter 17 - Bayesian Reasoning In The Twilight Zone
 * ====================================================
 */

fn main() {
    let mut num_correct_guesses = 0;

    /* Hypothesis - The seer can predict the future. */
    let p_data_true: f64 = 1.0;
    let p_true: f64 = 0.000001;

    /* Null Hypothesis - The seer just got lucky. */
    let p_data_lucky: f64 = 0.5;
    let p_lucky: f64 = 1.0 - p_true;

    loop {
        num_correct_guesses += 1;
        let posterior_odds = (p_true * p_data_true.powi(num_correct_guesses))
            / (p_lucky * p_data_lucky.powi(num_correct_guesses));

        println!(
            "Guess {:4} : Odds {:.08}",
            num_correct_guesses, posterior_odds
        );

        if posterior_odds > 150.0 {
            println!(
                "\n{} correct guesses to give overwhelming evidence of predictive powers.",
                num_correct_guesses
            );
            break;
        }
    }

    /* The odds of a coin with only heads. */
    calculate_posterior_odds(
        "A Coin With Only Heads (Trustworthy)",
        0.01,
        1.0,
        0.99,
        0.5_f64.powi(10),
    );
    calculate_posterior_odds(
        "A Coin With Only Heads (Prankster)",
        0.5,
        1.0,
        0.99,
        0.5_f64.powi(10),
    );

    /* How many heads to cast doubt on the flipper? */
    find_coin_flips_to_beat_odds("A Trustworthy Person", 1.0 / 10_000.0, 0.99, 1.0);
    find_coin_flips_to_beat_odds("A Untrustworthy Person", 0.1, 0.99, 1.0);

    /* Find prior belief of a cheater given strong posterior odds of a cheater. */
    println!("\nPrior Odds of a Cheater = {}", 100.0 * 0.5_f64.powi(4));
}

fn calculate_posterior_odds(title: &str, p_h1: f64, p_d_h1: f64, p_h0: f64, p_d_h0: f64) {
    let bayes_factor = p_d_h1 / p_d_h0;
    let prior_odds = p_h1 / p_h0;
    let posterior_odds = prior_odds * bayes_factor;

    println!(
        concat!(
            "\n{}",
            "\n{}",
            "\nBayes Factor   = {}",
            "\nPrior Odds     = {}",
            "\nPosterior Odds = {}",
        ),
        title,
        std::iter::repeat("=").take(title.len()).collect::<String>(),
        bayes_factor,
        prior_odds,
        posterior_odds,
    );
}

fn find_coin_flips_to_beat_odds(title: &str, p_h1: f64, p_h0: f64, odds: f64) {
    let mut coin_flips = 0;

    loop {
        coin_flips += 1;
        let posterior_odds = (p_h1 / p_h0) * (1.0 / 0.5_f64.powi(coin_flips));

        if posterior_odds > odds {
            println!(
                concat!(
                    "\n{}",
                    "\n{}",
                    "\nPosterior Odds = {}",
                    "\nCoin Flips     = {}",
                ),
                title,
                std::iter::repeat("=").take(title.len()).collect::<String>(),
                posterior_odds,
                coin_flips
            );
            break;
        }
    }
}
