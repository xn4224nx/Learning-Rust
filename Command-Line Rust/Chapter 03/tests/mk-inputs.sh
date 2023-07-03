#!/usr/bin/env bash

# define the input directory
OUTDIR="./inputs"

# Test if the input directory exists and create it if it doesn't exist
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

# Create files with text in them
touch $OUTDIR/empty.txt
echo "The quick brown fox jumps over the lazy dog." > $OUTDIR/fox.txt
echo -e "Don't worry, spiders,\nI keep house\ncasually." > $OUTDIR/spiders.txt
echo -e "The bustle in a house\nThe morning after death\nIs solemnest of industries\nEnacted upon earth,—\n\nThe sweeping up the heart,\nAnd putting love away\nWe shall not want to use again\nUntil eternity." > $OUTDIR/the-bustle.txt

