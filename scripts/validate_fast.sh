#!/usr/bin/env bash
set -euo pipefail

# Validacion rapida para Pull Requests diarios.
# No ejecuta compilacion nativa completa ni pruebas para mantener bajo el costo del CI.
echo "Validando estilo del repositorio"
bash scripts/style_check.sh

echo "Validando formato Rust"
cargo fmt --all -- --check

echo "Validacion rapida completa"
