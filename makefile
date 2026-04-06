MODULE ?= basic1

.DEFAULT_GOAL := help

.PHONY: help build run clean fmt fmt-check lint clippy doc doc-open test check

help: ## 💡 Show this help message
	@echo ""
	@echo "  🎮 \033[1;36mArgh Engine\033[0m"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""

build: ## 🔨 Build all crates
	cargo build --workspace

run: ## 🚀 Run an example (MODULE=basic1)
	cargo run --bin $(MODULE)

test: ## 🧪 Run all tests
	cargo test --workspace

check: ## ✅ Type check all crates
	cargo check --workspace

fmt: ## 🎨 Format all code
	cargo fmt --all

fmt-check: ## 🔍 Check formatting (CI)
	cargo fmt --all -- --check

clippy: ## 📎 Run clippy lints
	cargo clippy --workspace -- -D warnings

lint: fmt-check clippy ## 🧹 Run all lints (fmt + clippy)

doc: ## 📚 Generate documentation
	cargo doc --workspace --no-deps

doc-open: ## 📖 Generate and open documentation
	cargo doc --workspace --no-deps --open

clean: ## 🗑️  Clean build artefacts
	cargo clean