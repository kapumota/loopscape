#!/usr/bin/env bash
set -euo pipefail

# Validacion media usada antes de abrir Pull Requests importantes.
# Incluye compilacion nativa y pruebas, pero no compila WebAssembly.
modo="${1:-}"

if [[ "$modo" == "--full" ]]; then
    echo "El modo --full se mantiene por compatibilidad. Use make validate-full como flujo recomendado."
fi

echo "Ejecutando validacion rapida"
bash scripts/validate_fast.sh

echo "Verificando compilacion nativa"
cargo check --locked --all-targets

echo "Ejecutando pruebas nativas"
cargo test --locked --all-targets --no-fail-fast

if [[ "$modo" == "--full" ]]; then
    echo "Ejecutando Clippy"
    cargo clippy --all-targets -- -D warnings

    echo "Ejecutando validacion web"
    bash scripts/validate_web.sh
fi

echo "Validacion media completa"
