prepare:
	cargo fmt && cargo clippy --fix

release:
	cargo release patch --execute

test:
	@cargo nextest run --nocapture
