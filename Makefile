SHELL := /bin/bash

.PHONY: setup style-check fmt fmt-check check test run web-serve web-build validate clean clean-deep proxy-install proxy-run

setup:
	@echo "Preparando entorno de Loopscape"
	rustup target add wasm32-unknown-unknown
	cargo install trunk --locked

style-check:
	@bash scripts/style_check.sh

fmt:
	@echo "Aplicando formato Rust"
	cargo fmt --all

fmt-check:
	@echo "Verificando formato Rust"
	cargo fmt --all -- --check

check:
	@echo "Verificando compilacion nativa"
	cargo check --all-targets

test:
	@echo "Ejecutando pruebas"
	cargo test --all-targets

run:
	@echo "Ejecutando Loopscape en modo nativo"
	cargo run

web-serve:
	@echo "Ejecutando Loopscape en navegador"
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve

web-build:
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk build

web-build-release:
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk build --release

clippy:
	cargo clippy --all-targets -- -D warnings

validate-local:
	$(MAKE) fmt-check
	$(MAKE) check
	$(MAKE) test
	$(MAKE) clippy
	$(MAKE) web-build
