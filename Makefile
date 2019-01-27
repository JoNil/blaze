all:
	cargo build --release && sudo target/release/pyre 

.PHONY: all