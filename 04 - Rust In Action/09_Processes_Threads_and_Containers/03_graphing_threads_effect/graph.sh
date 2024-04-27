#!/bin/bash

n_simul=10000

cargo build -q --release &> /dev/null

# Run the sleep simulation if the file doesn't exist
if [ ! -e "sleep.tsv" ]; then
    ./target/release/graphing_threads_effect sleep $n_simul &> sleep.tsv
fi

# Run the sleep simulation if the file doesn't exist
if [ ! -e "spin.tsv" ]; then
    ./target/release/graphing_threads_effect spin $n_simul &> spin.tsv
fi

# Graph the simulation data
gnuplot plt_spin.gnuplot
gnuplot plt_sleep.gnuplot

# Clean up
cargo clean -q

