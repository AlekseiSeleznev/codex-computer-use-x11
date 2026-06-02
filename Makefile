.PHONY: fmt check test

fmt:
	cargo fmt -- --check

check:
	cargo check

test:
	cargo test
