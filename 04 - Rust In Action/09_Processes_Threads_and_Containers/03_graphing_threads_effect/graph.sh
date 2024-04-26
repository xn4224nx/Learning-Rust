#!/bin/bash

n_simul=10000
outfile=./alloc.tsv

cargo build -q --release &> /dev/null

# Run the simulation if the file doesn't exist
if [ ! -e "$outfile" ]; then
    ./target/release/graphing_threads_effect placeholder $n_simul &> $outfile
fi

# Graph the simulation data
gnuplot file.gnuplot

# Clean up
cargo clean -q
#rm $outfile

