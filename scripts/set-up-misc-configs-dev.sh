#!/bin/bash

current_directory=$(pwd)

# Check if the current directory is 'scripts'
if [[ "$current_directory" == *"/scripts"* ]]; then
    # Change directory to the parent directory ('linalg')
    builtin cd "$(dirname "$current_directory")"
fi

# Check if .vscode directory does not exist
if [ ! -d ".vscode" ]; then
    mkdir .vscode
fi

builtin cd ./docs/miscs/

cp launch.json settings.json ../../.vscode/