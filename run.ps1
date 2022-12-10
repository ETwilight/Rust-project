#!/bin/bash
$commands =
"cd rust_kill",
"Start-Process cargo 'run 8000 true'",
"Start-Process cargo 'run 8001 false'",
"Start-Process cargo 'run 8002 false'",
"Start-Process cargo 'run 8003 false'",
"Start-Process cargo 'run 8004 false'",
"Start-Process cargo 'run 8005 false'"
# Run the commands using the iex cmdlet
foreach ($command in $commands) {
    iex $command
}
done