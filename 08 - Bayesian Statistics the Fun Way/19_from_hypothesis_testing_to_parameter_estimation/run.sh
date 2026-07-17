#!/bin/bash

# Build the rust binary
cargo build -q --release &> /dev/null

# Run the binary to create the data files
./target/release/from_hypothesis_testing_to_parameter_estimation

# Plot the graphs
gnuplot ./src/plot_bayes_factor.gnuplot
gnuplot ./src/plot_bayes_factor_expert.gnuplot
gnuplot ./src/plot_bayes_factor_expert_norm.gnuplot
gnuplot ./src/plot_alt_bayes_factor.gnuplot
gnuplot ./src/plot_alt_bayes_factor_norm.gnuplot

# Clean up the rust binaries
cargo clean -q
