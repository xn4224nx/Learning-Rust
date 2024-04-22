#!/bin/bash

cargo build -q

for i in {1..10}
do
 echo -e "\nIteration $i"
 ./target/debug/trait_objects
done

cargo clean -q
