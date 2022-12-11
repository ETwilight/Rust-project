#!/bin/bash
$urls = "127.0.0.1:8007", "127.0.0.1:8001", "127.0.0.1:8002"

$commands =
"cd rust_kill",
"cargo run 8007",
"cargo run 8001",
"cargo run 50000",
"sleep 5.0",
# prevent 8002, 8013
# Run the commands using the iex cmdlet

for command in "$commands"; do
    $command
done

for url in "$urls"; do
    open -a "Safari" "$url"
done

