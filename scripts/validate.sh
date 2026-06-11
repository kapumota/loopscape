#!/usr/bin/env bash
set -euo pipefail

# Validacion local usada antes de abrir un Pull Request.
echo "Validando estilo del repositorio"
bash scripts/style_check.sh

echo "Validando formato"
cargo fmt --all -- --check

echo "Validando compilacion nativa"
cargo check --all-targets

echo "Ejecutando pruebas"
cargo test --all-targets

echo "Compilando WebAssembly"
trunk build --release

echo "Validacion completa"
