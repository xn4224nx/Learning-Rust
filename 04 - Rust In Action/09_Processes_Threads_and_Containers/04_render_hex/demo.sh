#!/bin/bash

cargo build -q

cargo run -- $(
    echo $RANDOM |
    sha1sum |
    cut -f1 -d' '
)

cargo clean -q
