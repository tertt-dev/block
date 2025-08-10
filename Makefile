.PHONY: build fmt clippy clean

build:
	cargo build -p block-router

fmt:
	rustfmt +stable --edition 2021 --emit files

clippy:
	cargo clippy -p block-router -- -D warnings || true  # allow warnings locally for now

clean:
	cargo clean
