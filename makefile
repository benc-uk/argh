MODULE ?= basic1

.PHONY: run

run:
	cargo run --manifest-path examples/$(MODULE)/Cargo.toml