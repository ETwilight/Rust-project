#!/bin/bash
$urls = "127.0.0.1:8007", "127.0.0.1:8001", "127.0.0.1:8002"

$commands =
"cd rust_kill",
"Start-Process cargo 'run 8007'",
"Start-Process cargo 'run 8001'",
"Start-Process cargo 'run 50000'",
"Start-Sleep -Seconds 5.0"
# prevent 8002, 8013
# Run the commands using the iex cmdlet
foreach ($command in $commands) {
    iex $command
}

foreach ($url in $urls) {
    Start-Process chrome.exe -ArgumentList "-new-tab $url"
}