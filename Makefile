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
	trunk serve

web-build:
	@echo "Compilando Loopscape para WebAssembly"
	trunk build --release

validate: style-check fmt-check check test web-build
	@echo "Validacion completa"

clean:
	@bash scripts/clean.sh

clean-deep: clean
	@echo "Eliminando dependencias locales del proxy"
	rm -rf proxy/node_modules proxy/package-lock.json

proxy-install:
	@echo "Instalando dependencias del proxy local"
	cd proxy && npm install

proxy-run:
	@echo "Ejecutando proxy local"
	cd proxy && node local.js
