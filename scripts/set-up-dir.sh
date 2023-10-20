#!/bin/bash

# Get the current working directory
current_directory=$(pwd)

# Check if the current directory is 'scripts'
if [[ "$current_directory" == *"/scripts"* ]]; then
    # Change directory to the parent directory ('linalg')
    builtin cd "$(dirname "$current_directory")"
fi

# Now you are in the 'linalg' directory
# Add your other script commands here
# ...

# Execute your script commands

# ...
# Note: If the current directory was 'scripts', the script has already changed to 'linalg'.
# If the current directory was already 'linalg', the script will execute from here directly.