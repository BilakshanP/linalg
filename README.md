<h1 align="center"><b>linalg | Python X Rust</b></h1>
<h4 align="center">A Python linear algebra library, written in Rust, using PyO3 binding and developed in Maturin.</h4>

<h6 align="center">Aimed to be simple to use and targeted towards non-data-science related tasks.</h6>
<br>


### Steps to make a local copy of the branch:
<hr>

Open your bash terminal and paste the following command(s):
> ```sh
> git clone https://github.com/SketchX7/linalg
> cd linalg
> bash ./set-up.sh
> ```


## Introduction
+ ### Makefile Bash Shortcuts
    + `make run` - executes the python code written in the `tests/main.py` file. A shortcut for executing `python3 ./tests/main.py`.
    + `make build` - builds and installs the library to the testing virtual environment. A shortcut for executing `maturin develop`.
    + `make execute` - runs `make build` and `make run` in the background.

+ ### Maturin
    + `maturin develop` - builds and installs the library to the testing virtual environment (unoptimized + debug info).
    + `maturin develop --release` - builds and installs the library to the testing virtual environment (optimized + release).
    + `maturin build` - builds the library and saves the installation file (`.whl`) to `linalg/target/wheels/` folder (unoptimized + debug info).
    + `maturin build --release` - builds the library and saves the installation file (`.whl`) to `linalg/target/wheels/` folder (optimized + release).

+ ### Type Hinting/Code Completion
    + In `linalg/linalg/` you can define the `.pyi` (stub files) for IDEs code completion. This will be included into the final installation file (`.whl`).
    + More info to come.

+ ### Wheel file (```.whl```) Installation Procedure
    1. Download the `.whl` file from the `to be done` (or build it yourself).
    2. Move the `.whl` file to the virtual environment or folder of your choice.
    3. Execute `pip install linalg-abc..xyz.whl` in the bash shell.
