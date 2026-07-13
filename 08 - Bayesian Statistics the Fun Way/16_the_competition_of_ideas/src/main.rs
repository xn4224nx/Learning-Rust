/*
 * Chapter 16 - Introduction To The Bayes Factor And Posterior Odds: The Competition Of Ideas
 * ==========================================================================================
 */

fn main() {
    let dice_rolls = vec![6, 1, 3, 6, 4, 5, 6, 1, 2, 6];

    /* Hypothesis 1 - The dice rolls a 6 half the time. */
    let p_h1 = 1.0 / 3.0;
    let p_data_h1 = prob_of_data_hypth(&dice_rolls, &vec![0.1, 0.1, 0.1, 0.1, 0.1, 0.5]);

    /* Hypothesis 2 - The dice is fair */
    let p_h2 = 2.0 / 3.0;
    let p_data_h2 = prob_of_data_hypth(&dice_rolls, &vec![1.0 / 6.0; 6]);

    show_results(
        "Weighted Die or Fair Dice",
        p_h1,
        p_h2,
        p_data_h1,
        p_data_h2,
    );

    /* Hypothesis 1 - Earwax Impaction */
    let p_h1 = 0.037;
    let p_data_h1 = 0.63 * 0.55;

    /* Hypothesis 2 - Vestibular Schwannoma */
    let p_h2 = 0.000011;
    let p_data_h2 = 0.94 * 0.89;

    show_results(
        "Earwax or Vestibular Schwannoma",
        p_h1,
        p_h2,
        p_data_h1,
        p_data_h2,
    );

    /* Hypothesis 1 - The dice rolls a 6 half the time. */
    let p_h1 = 2.0 / 3.0;
    let p_data_h1 = prob_of_data_hypth(&dice_rolls, &vec![0.1, 0.1, 0.1, 0.1, 0.1, 0.5]);

    /* Hypothesis 2 - The dice is fair */
    let p_h2 = 1.0 / 3.0;
    let p_data_h2 = prob_of_data_hypth(&dice_rolls, &vec![1.0 / 6.0; 6]);

    show_results(
        "Weighted Dice or Fair Die",
        p_h1,
        p_h2,
        p_data_h1,
        p_data_h2,
    );

    /* Hypothesis 1 - Vestibular Schwannoma */
    let p_h1 = 0.000011;
    let p_data_h1 = 0.94 * 0.89 * 0.49;

    /* Hypothesis 2 - Labyrinthitis */
    let p_h2 = 0.000035;
    let p_data_h2 = 0.3 * 0.28 * 0.98;

    show_results(
        "Vestibular Schwannoma or Labyrinthitis",
        p_h1,
        p_h2,
        p_data_h1,
        p_data_h2,
    );
}

fn strength_of_evidence(posterior_odds: f64) -> String {
    return String::from(if !posterior_odds.is_normal() || posterior_odds < 1.0 {
        "Not viable!"
    } else if posterior_odds < 3.0 {
        "Interesting, but nothing conclusive."
    } else if posterior_odds < 20.0 {
        "Looks like we’re on to something."
    } else if posterior_odds < 150.0 {
        "Strong evidence in favor of the hypothesis."
    } else {
        "Overwhelming evidence."
    });
}

/// What is the probability of the observed dice rolls given the hypothosised
/// probabilities of each number on the dice?
fn prob_of_data_hypth(data: &Vec<usize>, hyp_prop_dist: &Vec<f64>) -> f64 {
    let mut prob = 1.0;

    for d_idx in 0..data.len() {
        prob *= hyp_prop_dist[data[d_idx] - 1];
    }
    return prob;
}

fn show_results(
    experiment: &str,
    p_h1_fn: f64,
    p_h2_fn: f64,
    p_data_h1_fn: f64,
    p_data_h2_fn: f64,
) {
    let post_ratio_fn = (p_h1_fn * p_data_h1_fn) / (p_h2_fn * p_data_h2_fn);

    println!(
        concat!(
            "\n{}",
            "\n{}",
            "\nP(H1) = {:.6}",
            "\nP(H2) = {:.6}",
            "\nP(D | H1) = {:.10}",
            "\nP(D | H2) = {:.10}",
            "\nPosterior Ratio = {:.2}",
            "\nStrength of Posterior Ratio: '{}'\n"
        ),
        experiment,
        std::iter::repeat('=')
            .take(experiment.len())
            .collect::<String>(),
        p_h1_fn,
        p_h2_fn,
        p_data_h1_fn,
        p_data_h2_fn,
        post_ratio_fn,
        strength_of_evidence(post_ratio_fn)
    );
}
