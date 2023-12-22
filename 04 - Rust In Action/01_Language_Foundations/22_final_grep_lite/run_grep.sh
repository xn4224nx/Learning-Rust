#!/bin/bash

# Build & Run the Program
cargo build >/dev/null 2>&1

# Run the program with the file
cargo run -q -- Welsh ./data/a_welsh_testament.txt > test.txt

# Removed the compiled program
cargo clean >/dev/null 2>&1
