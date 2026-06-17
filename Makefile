SHELL := /bin/bash

.PHONY: setup setup-native setup-web style-check fmt fmt-check check test run web-serve web-build web-build-release clippy validate-fast validate validate-web validate-full clean clean-deep proxy-install proxy-run test-core test-deterministic smoke-native run-script-example

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

run:
	@echo "Ejecutando Loopscape en modo nativo"
	cargo run

setup: setup-native

setup-native:
	@echo "Preparando entorno nativo de Loopscape"
	@rustc --version
	@cargo --version

setup-web:
	@echo "Preparando entorno web de Loopscape"
	rustup target add wasm32-unknown-unknown
	cargo install trunk --locked

test:
	@echo "Ejecutando pruebas nativas"
	cargo test --all-targets --no-fail-fast

web-serve:
	@echo "Sirviendo Loopscape en navegador"
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve

web-build:
	@echo "Compilando Loopscape para navegador"
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk build

web-build-release:
	@echo "Compilando Loopscape para navegador en modo release"
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk build --release

clippy:
	@echo "Ejecutando Clippy"
	cargo clippy --all-targets -- -D warnings

validate-fast:
	@bash scripts/validate_fast.sh

validate:
	@bash scripts/validate.sh

validate-web:
	@bash scripts/validate_web.sh

validate-full:
	$(MAKE) validate
	$(MAKE) validate-web
	$(MAKE) clippy

validate-multiagent:
	@bash scripts/validate_multiagent.sh

clean:
	@bash scripts/clean.sh

clean-deep:
	@bash scripts/clean.sh
	cargo clean

proxy-install:
	cd proxy && npm install

proxy-run:
	cd proxy && npm run dev

test-core:
	@echo "Ejecutando pruebas rapidas del nucleo"
	cargo test --locked --lib core

test-deterministic:
	@echo "Ejecutando pruebas deterministas del nucleo"
	cargo test --locked --lib deterministic

smoke-native:
	@echo "Ejecutando prueba de humo nativa"
	cargo run -- --smoke --seed 123 --ticks 10

run-script-example:
	@echo "Ejecutando ejemplo DSL de rescate"
	cargo run -- --script examples/rescate.loop --seed 123 --ticks 50
