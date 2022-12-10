#!/usr/bin/env sh

urls=(
"127.0.0.1:8000"
"127.0.0.1:8001"
"127.0.0.1:8002"
"127.0.0.1:8003"
"127.0.0.1:8004"
"127.0.0.1:8005"
)

commands=(
"cd rust_kill"
"cargo run 8000 true"
"cargo run 8001 false"
"cargo run 8002 false"
"cargo run 8003 false"
"cargo run 8004 false"
"cargo run 8005 false"
"sleep 3.0"
)

for command in "${commands[@]}"
do
    $command
done

for url in "${urls[@]}"
do
    xdg-open "$url"
done
