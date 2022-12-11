#!/bin/bash
$urls = "127.0.0.1:8000", "127.0.0.1:8001", "127.0.0.1:8002", "127.0.0.1:8003", "127.0.0.1:8004", "127.0.0.1:8005"

$commands =
"cd rust_kill",
"Start-Process cargo 'run 8000'",
"Start-Process cargo 'run 8001'",
"Start-Process cargo 'run 8002'",
"Start-Process cargo 'run 8003'",
"Start-Process cargo 'run 8004'",
"Start-Process cargo 'run 8005'",
"Start-Sleep -Seconds 3.0"
# Run the commands using the iex cmdlet
foreach ($command in $commands) {
    iex $command
}

foreach ($url in $urls) {
    Start-Process chrome.exe -ArgumentList "-new-tab $url"
}