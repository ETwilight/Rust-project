#!/usr/bin/env sh

# Define the commands to run
commands="cd rust_kill; cargo run 8000 true; cargo run 8001 false; cargo run 8002 false; cargo run 8003 false; cargo run 8004 false; cargo run 8005 false"

# Change to the rust_kill directory
cd rust_kill

# Run the commands using the & operator
$commands &