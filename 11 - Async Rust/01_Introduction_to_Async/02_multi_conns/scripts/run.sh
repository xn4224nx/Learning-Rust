#!/usr/bin/env bash

# Navigate to the root directory.
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd "$SCRIPTPATH"
cd ..

# Run all four processes in the background and extract the PIDs.
./server_bin &
pid0=$!
./server_bin &
pid1=$!
./server_bin &
pid2=$!
./server_bin &
pid3=$!

# Wait for them to finish and collect the exit codes.
wait $pid0
exit_code0=$?
wait $pid1
exit_code1=$?
wait $pid2
exit_code2=$?
wait $pid3
exit_code3=$?

# Show the exit codes
echo "
    Task 1 (PID $pid0) had exit code $exit_code0
    Task 2 (PID $pid1) had exit code $exit_code1
    Task 3 (PID $pid2) had exit code $exit_code2
    Task 4 (PID $pid3) had exit code $exit_code3
"
