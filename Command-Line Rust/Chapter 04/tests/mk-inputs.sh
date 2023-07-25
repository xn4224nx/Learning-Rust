#!/usr/bin/bash/env bash

# Define the directory to create the files in
OUT_DIR="./inputs"

# Test if the directory exists and if not create it
[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

# Create an empty file
touch $OUT_DIR/empty.txt

# A text file with one line of text.
echo "Öne line, four words." > $OUT_DIR/one.txt

# A text file with two lines of text
echo -e "Two lines.\nFour words." > $OUT_DIR/two.txt

# Three lines but with windows endings
echo -e "Three\nlines,\nfour words." > $OUT_DIR/three.txt
unix2dos -q $OUT_DIR/three.txt

# Ten lines in the file
echo -e "one\ntwo\nthree\nfour\nfive\nsix\nseven\neight\nnine\nten" \
> $OUT_DIR/ten.txt

