#!/bin/bash

# Run the allocation timing script
cargo run -q 2> alloc.tsv

# Generate a plot in GNU plot
gnuplot file.gnuplot

# Clean up
cargo clean
rm alloc.tsv
