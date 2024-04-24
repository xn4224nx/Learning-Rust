#!/bin/bash

cargo build -q

for i in {1..10}
do
 ./target/debug/mac_addr_gen
done

cargo clean -q
