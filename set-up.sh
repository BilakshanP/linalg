python3 -m venv .env
source .env/bin/activate # deactivate
echo To exit the virtual environment type 'deactivate' in the shell.

pip install maturin
echo Select PyO3 in next prompt.
maturin init

mkdir tests
touch tests/main.py