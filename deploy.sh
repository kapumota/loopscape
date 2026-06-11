#!/usr/bin/env bash
set -euo pipefail

# Script de construccion web para Loopscape.
echo "Compilando Loopscape para WebAssembly"

if ! command -v trunk > /dev/null 2>&1; then
    echo "Instalando Trunk"
    cargo install trunk --locked
fi

trunk build --release

echo "Compilacion completa. Los archivos quedaron en dist/"
echo "Para GitHub Pages, publica dist/ desde la rama o workflow correspondiente."
echo "Para prueba local, ejecuta trunk serve."
