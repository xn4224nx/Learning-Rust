#!/usr/bin/env bash

# Navigate to the root directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd "$SCRIPTPATH"
cd ..

# Build the server and client binaries
cargo build -r
cd server && cargo build -r && cd ..

# Copy the recently created binaries
cp target/release/connection_bin ./
cp server/target/release/server_bin ./

# Clean up the compilation files
cargo clean
cd server && cargo clean
