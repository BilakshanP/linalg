.SILENT: build run execute predump

# @ to silent, - to ignore non-zero errors

run:
	./.env/bin/python ./tests/main.py

build:
	maturin develop

execute: # make build && make run
	maturin develop 2>> /dev/null
	./.env/bin/python ./tests/main.py

predump:
	touch dump.asm
	rm dump.asm
	touch dump.asm

dump:
	@make idump
	@echo Dumping debug...
	@objdump -wDr -Mintel ./target/debug/LinAlg >> dump.asm
	@echo Dumping done.

rdump:
	@make idump
	@echo Dumping release...
	@objdump -wDr -Mintel ./target/release/LinAlg >> dump.asm
	@echo Dumping done.

clean:
	cargo clean
	rm dump.asm