#!/usr/bin/env bash
set -euo pipefail

echo "Loopscape - preparacion de entorno"
echo "-----------------------------------"

# Verifica Rust.
if ! command -v rustc > /dev/null 2>&1; then
    echo "Rust no esta instalado. Instala Rust desde https://rustup.rs/"
    exit 1
fi

# Instala el target WASM si falta.
if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
    echo "Instalando target wasm32-unknown-unknown"
    rustup target add wasm32-unknown-unknown
fi

# Instala Trunk si falta.
if ! command -v trunk > /dev/null 2>&1; then
    echo "Instalando Trunk"
    cargo install trunk --locked
fi

# Node solo es necesario para el proxy local.
if ! command -v node > /dev/null 2>&1; then
    echo "Node.js no esta instalado. Solo es necesario para el proxy local LLM."
fi

echo ""
echo "Entorno preparado"
echo ""
echo "Opciones de ejecucion:"
echo "   1. Nativo: cargo run"
echo "   2. Web local: trunk serve"
echo "   3. Web build: trunk build --release"
echo "   4. Con proxy: trunk serve y node proxy/local.js"
echo ""
echo "Despliegue:"
echo "   - GitHub Pages: ./deploy.sh"
echo "   - Netlify: usa netlify.toml"
echo "   - Vercel: usa vercel.json"
echo "   - Cloudflare: cd proxy && wrangler deploy"
