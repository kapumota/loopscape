#!/usr/bin/env bash
set -euo pipefail

# Validacion web manual.
# Instala el target WebAssembly y Trunk solo cuando se solicita de forma explicita.
echo "Preparando target WebAssembly"
rustup target add wasm32-unknown-unknown

if ! command -v trunk > /dev/null 2>&1; then
    echo "Instalando Trunk"
    cargo install trunk --locked
fi

echo "Compilando WebAssembly en modo release"
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk build --release

echo "Validacion web completa"
