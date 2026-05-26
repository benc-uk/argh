EXAMPLE ?= teapots

.DEFAULT_GOAL := help

.PHONY: help build build-win release release-win run clean fmt fmt-check lint clippy doc doc-open test check wasm-build wasm-serve site

help: ## 💡 Show this help message
	@echo ""
	@echo "  🎮 \033[1;36mArgh Engine\033[0m"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""

build: ## 🔨 Build all crates
	cargo build --workspace 

build-win: ## 🔨 Build all crates for Windows x64
	cargo build --workspace --target x86_64-pc-windows-gnu

release: ## 🚀 Build all crates (release)
	cargo build --workspace --release

release-win: ## 🚀 Build all crates for Windows x64 (release)
	cargo build --workspace --release --target x86_64-pc-windows-gnu

run-example: ## 🚀 Run an example as a desktop app
	cargo run --bin $(EXAMPLE)

test: ## 🧪 Run all tests
	cargo test --lib

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
	rm -rf examples/web_wasm/pkg
	rm -rf site

wasm-build: ## 🕸️  Build the web_wasm example with wasm-pack
	wasm-pack build examples/web_wasm --target web --out-dir pkg --no-typescript --no-pack

wasm-serve: wasm-build ## 🌐 Build and serve the web_wasm example on http://localhost:8000
	@echo "Serving at http://localhost:8000 (Ctrl+C to stop)"
	cd examples/web_wasm && python3 -m http.server 8000

site: wasm-build doc ## 📚 Build the project site combining docs and WASM example(s)
	mkdir -p site/examples/web_wasm
	cp misc/pages-index.html site/index.html
	cp -r target/doc/ site/
	echo '<meta http-equiv="refresh" content="0;url=argh/index.html">' > site/doc/index.html
	cp -r examples/web_wasm/pkg site/examples/web_wasm/
	cp examples/web_wasm/index.html site/examples/web_wasm/