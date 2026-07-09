/*
 * Chapter 8 - The Prior, Likelihood, and the Posterior of Bayes' Theorem
 * ======================================================================
 */

fn main() {
    let p_robbed = 0.001;
    let p_rob_evidence = 0.3;
    let un_norm_posterior_h1 = p_robbed * p_rob_evidence;
    let un_norm_posterior_h2 = 1.0 / (2000.0 * 30.0 * 365.0);

    println!(
        "Posterior Ratio H1 / H2 = {}",
        un_norm_posterior_h1 / un_norm_posterior_h2
    );

    let p_rob_evidence_revised = 0.03;

    println!(
        "Posterior Ratio H1 / H2 = {}",
        p_robbed * p_rob_evidence_revised / un_norm_posterior_h2
    );

    println!(
        "P(Robbed) = {}",
        un_norm_posterior_h2 / (p_rob_evidence_revised)
    );
}
