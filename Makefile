all:
	cargo build --release && sudo target/release/rpi_gpu_game

.PHONY: all