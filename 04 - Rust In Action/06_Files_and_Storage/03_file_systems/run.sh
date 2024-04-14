#!/bin/bash

tmp_file_path="test.txt"

# Create the file
touch $tmp_file_path

# Change the file permissions so it can't be read
chmod u-r $tmp_file_path

cargo run $tmp_file_path
cargo clean

rm $tmp_file_path


