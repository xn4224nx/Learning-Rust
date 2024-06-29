#!/bin/bash

mkdir ./data

# Create the CSV file
cargo run --release > ./data/cat_data.csv