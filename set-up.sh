#!/bin/bash

pip install maturin -U


# Check if the virtual environment does not exist
if [ ! -d ".env" ]; then
    python3 -m venv .env
    source .env/bin/activate
    echo "To exit the virtual environment type 'deactivate' in the shell."
fi

# Check if tests directory does not exist
if [ ! -d "tests" ]; then
    mkdir tests
    touch tests/main.py
fi

# Check if src directory does not exist
if [ ! -d "src" ]; then
    mkdir src
    touch src/main.rs
fi

# Check if maturin.toml does not exist
if [ ! -f "pyproject.toml" ]; then
    echo "Select PyO3 in the next prompt."
    maturin init
fi
