all:
	cargo build --release && sudo target/release/blaze 

.PHONY: all