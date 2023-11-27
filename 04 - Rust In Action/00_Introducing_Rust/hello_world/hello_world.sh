#!/bin/bash

# Do all of this in a temp directory
cd $TMP

# Create a project and run it
cargo new hello
cd hello
cargo run -q

# Remove the project
cd ..
rm -rf hello
