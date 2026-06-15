#!/usr/bin/env bash
set -euo pipefail

# Script de construccion web para Loopscape.
# No publica en GitHub Pages ni empuja cambios a ramas remotas.
echo "Compilando Loopscape para WebAssembly"

rustup target add wasm32-unknown-unknown

if ! command -v trunk > /dev/null 2>&1; then
    echo "Instalando Trunk"
    cargo install trunk --locked
fi

RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk build --release

echo "Compilacion completa. Los archivos quedaron en dist/"
echo "Publica dist/ manualmente en Netlify, Vercel, Cloudflare Pages u otro hosting estatico."
echo "Para prueba local, ejecuta make web-serve."
