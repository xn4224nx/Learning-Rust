#!/usr/bin/bash

declare -a urls=("www.bbc.co.uk" "docs.rs" "stackoverflow.com")

cargo build -q

# Loop over the array and resolve the addresses
for i in "${urls[@]}" 
do
   ./target/debug/dns_resolver "$i"
done

cargo clean -q
