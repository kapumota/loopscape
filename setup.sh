#!/usr/bin/env bash
set -euo pipefail

echo "Preparando entorno de Loopscape"

if ! command -v cargo > /dev/null 2>&1; then
    echo "Error: cargo no esta instalado"
    exit 1
fi

echo "Rust detectado"
rustc --version
cargo --version

# Node solo es necesario para el proxy local.
if ! command -v node > /dev/null 2>&1; then
    echo "Aviso: node no esta instalado. El proxy local no estara disponible."
fi

echo ""
echo "Entorno nativo preparado"
echo ""
echo "Opciones de ejecucion:"
echo "   1. Nativo: cargo run"
echo "   2. Validacion rapida: make validate"
echo "   3. Web local: make setup-web y make web-serve"
echo "   4. Web build: make setup-web y make web-build-release"
echo "   5. Con proxy: make proxy-run"
