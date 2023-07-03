#!/usr/bin/env bash

# define the output directory
OUTDIR="./outputs"
INDIR="./inputs"

# Test if the input directory exists and create it if it doesn't exist
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

# Create the output files using the cat command
cat $INDIR/empty.txt > $OUTDIR/empty.txt
cat $INDIR/fox.txt > $OUTDIR/fox.txt
cat -n $INDIR/spiders.txt > $OUTDIR/spiders.txt
cat -n $INDIR/the-bustle.txt > $OUTDIR/n-the-bustle.txt
cat -b $INDIR/the-bustle.txt > $OUTDIR/b-the-bustle.txt
cd $INDIR
cat -n empty.txt fox.txt spiders.txt the-bustle.txt > ../$OUTDIR/all.txt

