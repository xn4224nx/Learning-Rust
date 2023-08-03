#!/usr/bin/env bash

# define the output directory
OUTDIR="./expected"

# Test if the output directory exists and create it if it doesn't exist
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

# Create files with text in them
echo "Hello there" > $OUTDIR/echo_output_0.txt
echo "Hello" "there" > $OUTDIR/echo_output_1.txt
echo -n "Hello there" > $OUTDIR/echo_output_2.txt
echo -n "Hello" "there" > $OUTDIR/echo_output_3.txt

