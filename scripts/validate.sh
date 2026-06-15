#!/usr/bin/env bash
set -euo pipefail

# Validacion local rapida usada antes de abrir un Pull Request.
# Por defecto no compila WebAssembly para mantener el flujo diario liviano.
modo="${1:-}"

echo "Validando estilo del repositorio"
bash scripts/style_check.sh

echo "Validando formato Rust"
cargo fmt --all -- --check

echo "Validando compilacion nativa"
cargo check --all-targets

echo "Ejecutando pruebas nativas"
cargo test --all-targets --no-fail-fast

if [[ "$modo" == "--full" ]]; then
    echo "Ejecutando Clippy"
    cargo clippy --all-targets -- -D warnings

    echo "Preparando target WebAssembly"
    rustup target add wasm32-unknown-unknown

    if ! command -v trunk > /dev/null 2>&1; then
        echo "Instalando Trunk"
        cargo install trunk --locked
    fi

    echo "Compilando WebAssembly"
    RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk build --release
fi

echo "Validacion completa"
