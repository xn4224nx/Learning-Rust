#!/bin/bash

cargo build -q

# The array of inputs to the program
declare -a prog_inputs=(
    "abc" 
    "abcd" 
    "abcde"
    "abcdef"
)

for i in "${prog_inputs[@]}"
do
   cargo run -q $i
done

cargo clean -q
